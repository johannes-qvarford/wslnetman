// Network module
pub mod docker;
pub mod windows;
pub mod wsl;

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
    pub ip_addresses: Vec<String>,
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
/// This function returns network interfaces from Windows, WSL, and Docker environments.
pub fn get_all_network_interfaces() -> Result<Vec<NetworkInterface>, Box<dyn std::error::Error>> {
    let mut all_interfaces = Vec::new();

    // Get Windows network interfaces (through WSL)
    match windows::get_network_interfaces() {
        Ok(interfaces) => all_interfaces.extend(interfaces),
        Err(e) => eprintln!("Error getting Windows network interfaces: {e}"),
    }

    // Get WSL network interfaces
    match wsl::get_network_interfaces() {
        Ok(interfaces) => all_interfaces.extend(interfaces),
        Err(e) => eprintln!("Error getting WSL network interfaces: {e}"),
    }

    Ok(all_interfaces)
}

/// Get active ports from the current system
///
/// This function returns active ports from either Windows or WSL
/// depending on the compilation target.
pub fn get_active_ports() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    // For demonstration purposes, we'll use WSL port discovery
    // In a real implementation, we would detect the platform and call the appropriate function
    wsl::get_active_ports()
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
