// Network module
pub mod windows;
pub mod wsl;
pub mod docker;
pub mod firewall;
pub mod routing;

/// Represents a network interface with its properties
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_addresses: Vec<String>,
    pub is_up: bool,
    pub is_loopback: bool,
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

/// Represents a Docker network with its properties
#[derive(Debug, Clone)]
pub struct DockerNetwork {
    pub name: String,
    pub driver: String,
    pub scope: String,
    pub subnet: String,
}

/// Represents a firewall rule with its properties
#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub name: String,
    pub enabled: String,
    pub direction: String,
    pub action: String,
    pub protocol: String,
    pub local_address: String,
    pub remote_address: String,
}

/// Represents a routing rule with its properties
#[derive(Debug, Clone)]
pub struct Route {
    pub destination: String,
    pub gateway: String,
    pub interface: String,
    pub metric: String,
}

/// Get network interfaces from the current system
/// 
/// This function returns network interfaces from either Windows or WSL
/// depending on the compilation target.
pub fn get_network_interfaces() -> Result<Vec<NetworkInterface>, Box<dyn std::error::Error>> {
    // For demonstration purposes, we'll use WSL network discovery
    // In a real implementation, we would detect the platform and call the appropriate function
    wsl::get_network_interfaces()
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
pub fn get_docker_networks() -> Result<Vec<DockerNetwork>, Box<dyn std::error::Error>> {
    docker::get_docker_networks()
}

/// Get firewall rules from the current system
/// 
/// This function returns firewall rules from either Windows or WSL
/// depending on the compilation target.
pub fn get_firewall_rules() -> Result<Vec<FirewallRule>, Box<dyn std::error::Error>> {
    // For demonstration purposes, we'll use WSL firewall rules
    // In a real implementation, we would detect the platform and call the appropriate function
    firewall::wsl::get_firewall_rules()
}

/// Get routing rules from the current system
/// 
/// This function returns routing rules from either Windows or WSL
/// depending on the compilation target.
pub fn get_routing_rules() -> Result<Vec<Route>, Box<dyn std::error::Error>> {
    // For demonstration purposes, we'll use WSL routing rules
    // In a real implementation, we would detect the platform and call the appropriate function
    routing::wsl::get_routing_rules()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_network_interfaces() {
        let interfaces = get_network_interfaces().unwrap();
        assert!(!interfaces.is_empty());
    }

    #[test]
    fn test_get_active_ports() {
        let ports = get_active_ports().unwrap();
        assert!(!ports.is_empty());
    }

    #[test]
    fn test_get_docker_networks() {
        let networks = get_docker_networks().unwrap();
        assert!(!networks.is_empty());
    }

    #[test]
    fn test_get_firewall_rules() {
        let rules = get_firewall_rules().unwrap();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_get_routing_rules() {
        let rules = get_routing_rules().unwrap();
        assert!(!rules.is_empty());
    }
}