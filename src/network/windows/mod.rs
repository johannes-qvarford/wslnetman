//! Windows network discovery implementation
//! 
//! This module provides functionality to discover network interfaces and ports on Windows systems.
//! Since this application runs in WSL, we'll simulate Windows network data for demonstration purposes.

use crate::network::{NetworkInterface, PortInfo};

/// Get network interfaces from Windows system
/// 
/// In a real implementation, this would use Windows APIs like GetAdaptersAddresses
/// or parse the output of ipconfig command.
pub fn get_network_interfaces() -> Result<Vec<NetworkInterface>, Box<dyn std::error::Error>> {
    // Simulate Windows network interfaces
    let interfaces = vec![
        NetworkInterface {
            name: "Ethernet".to_string(),
            ip_addresses: vec!["192.168.1.100".to_string(), "fe80::1234:5678:9abc:def0".to_string()],
            is_up: true,
            is_loopback: false,
        },
        NetworkInterface {
            name: "Wi-Fi".to_string(),
            ip_addresses: vec!["192.168.0.105".to_string(), "fe80::abcd:ef01:2345:6789".to_string()],
            is_up: true,
            is_loopback: false,
        },
        NetworkInterface {
            name: "Loopback".to_string(),
            ip_addresses: vec!["127.0.0.1".to_string(), "::1".to_string()],
            is_up: true,
            is_loopback: true,
        },
    ];
    
    Ok(interfaces)
}

/// Get active ports from Windows system
/// 
/// In a real implementation, this would use Windows APIs like GetTcpTable/GetUdpTable
/// or parse the output of netstat command.
pub fn get_active_ports() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    // Simulate Windows port information
    let ports = vec![
        PortInfo {
            process_id: "1234".to_string(),
            process_name: "chrome.exe".to_string(),
            protocol: "TCP".to_string(),
            port: "8080".to_string(),
            direction: "LISTEN".to_string(),
            network: "192.168.1.100".to_string(),
        },
        PortInfo {
            process_id: "5678".to_string(),
            process_name: "spotify.exe".to_string(),
            protocol: "UDP".to_string(),
            port: "5353".to_string(),
            direction: "LISTEN".to_string(),
            network: "0.0.0.0".to_string(),
        },
        PortInfo {
            process_id: "9012".to_string(),
            process_name: "ssh.exe".to_string(),
            protocol: "TCP".to_string(),
            port: "22".to_string(),
            direction: "ESTABLISHED".to_string(),
            network: "192.168.1.100".to_string(),
        },
    ];
    
    Ok(ports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_network_interfaces() {
        let interfaces = get_network_interfaces().unwrap();
        assert_eq!(interfaces.len(), 3);
        
        // Check first interface
        assert_eq!(interfaces[0].name, "Ethernet");
        assert_eq!(interfaces[0].ip_addresses.len(), 2);
        assert!(interfaces[0].is_up);
        assert!(!interfaces[0].is_loopback);
        
        // Check loopback interface
        assert_eq!(interfaces[2].name, "Loopback");
        assert!(interfaces[2].is_loopback);
    }

    #[test]
    fn test_get_active_ports() {
        let ports = get_active_ports().unwrap();
        assert_eq!(ports.len(), 3);
        
        // Check first port
        assert_eq!(ports[0].process_name, "chrome.exe");
        assert_eq!(ports[0].protocol, "TCP");
        assert_eq!(ports[0].port, "8080");
    }
}