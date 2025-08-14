// Network module
pub mod docker;
pub mod windows;

/// Represents the environment where a network interface originates
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkEnvironment {
    Windows,
    Wsl,
}

/// Represents a network interface with its properties and source environment
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub ipv4_addresses: Vec<String>,
    pub ipv6_addresses: Vec<String>,
    pub mac_address: Option<String>,
    pub is_up: bool,
    pub is_loopback: bool,
    pub environment: NetworkEnvironment, // New field to identify source
}

/// Represents an active port with its associated process information
#[derive(Debug, Clone)]
pub struct PortInfo {
    pub process_id: String,
    pub process_name: String,
    pub protocol: String,
    pub port: String,
    pub direction: String,
    pub network: String,
}

/// Represents a Docker network with its properties and source environment
#[derive(Debug, Clone)]
pub struct DockerNetwork {
    pub name: String,
    pub driver: String,
    pub scope: String,
    pub subnet: String,
}

/// Get network interfaces from all environments
///
/// This function returns network interfaces from Windows and WSL (via wsl.exe interop).
pub fn get_all_network_interfaces() -> Result<Vec<NetworkInterface>, Box<dyn std::error::Error>> {
    let mut all_interfaces = Vec::new();

    // Get Windows network interfaces
    match windows::get_network_interfaces() {
        Ok(interfaces) => all_interfaces.extend(interfaces),
        Err(e) => eprintln!("Error getting Windows network interfaces: {e}"),
    }

    // Get WSL network interfaces via wsl.exe
    match windows::get_wsl_network_interfaces() {
        Ok(interfaces) => all_interfaces.extend(interfaces),
        Err(e) => eprintln!("Error getting WSL network interfaces via wsl.exe: {e}"),
    }

    Ok(all_interfaces)
}

/// Get active ports from Windows
///
/// This function returns active ports from Windows.
pub fn get_active_ports() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    let ports = windows::get_active_ports()?;
    Ok(ports)
}

/// Filter ports associated with a specific network interface
///
/// This function filters ports based on matching IP addresses between the interface and port bindings.
pub fn filter_ports_for_interface(
    interface: &NetworkInterface,
    all_ports: &[PortInfo],
) -> Vec<PortInfo> {
    let mut filtered_ports = Vec::new();

    // Collect all IP addresses from the interface
    let mut interface_ips = interface.ipv4_addresses.clone();
    interface_ips.extend(interface.ipv6_addresses.clone());

    for port in all_ports.iter() {
        // Extract the IP address from the network field (format: "ip:port")
        let port_ip = if let Some(colon_pos) = port.network.rfind(':') {
            port.network[..colon_pos].to_string()
        } else {
            port.network.clone()
        };

        // Check if the port's network address matches any of the interface's IPs
        // Also include ports bound to 0.0.0.0 or :: (all interfaces)
        let matches = interface_ips.contains(&port_ip)
            || port_ip == "0.0.0.0"
            || port_ip == "::"
            || port_ip == "*";

        if matches {
            filtered_ports.push(port.clone());
        }
    }
    filtered_ports
}

/// Get Docker networks
///
/// This function returns Docker network information.
pub fn get_all_docker_networks() -> Result<Vec<DockerNetwork>, Box<dyn std::error::Error>> {
    let mut all_networks = Vec::new();

    // Get Docker networks
    match docker::get_docker_networks() {
        Ok(networks) => {
            // Convert docker::DockerNetwork to crate::network::DockerNetwork
            let converted_networks: Vec<DockerNetwork> = networks
                .into_iter()
                .map(|network| DockerNetwork {
                    name: network.name,
                    driver: network.driver,
                    scope: network.scope,
                    subnet: network.subnet,
                })
                .collect();
            all_networks.extend(converted_networks);
        }
        Err(e) => eprintln!("Error getting Docker networks: {e}"),
    }

    Ok(all_networks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_network_interfaces() {
        let interfaces = get_all_network_interfaces().unwrap();
        assert!(!interfaces.is_empty());
    }

    #[test]
    fn test_get_active_ports() {
        let ports = get_active_ports().unwrap();
        assert!(!ports.is_empty());
    }

    #[test]
    fn test_get_all_docker_networks() {
        let networks = get_all_docker_networks().unwrap();
        assert!(!networks.is_empty());
    }
}
