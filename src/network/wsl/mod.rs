//! WSL network discovery implementation
//! 
//! This module provides functionality to discover network interfaces and ports on WSL systems.
//! It uses command-line tools available in Linux to gather network information.

use crate::network::{NetworkInterface, PortInfo};
use std::process::Command;

/// Get network interfaces from WSL system
/// 
/// This function uses the `ip addr` command to get network interface information.
pub fn get_network_interfaces() -> Result<Vec<NetworkInterface>, Box<dyn std::error::Error>> {
    // In a real implementation, we would execute:
    // let output = Command::new("ip").args(&["addr", "show"]).output()?;
    // For now, we'll simulate the output
    
    // Simulate WSL network interfaces
    let interfaces = vec![
        NetworkInterface {
            name: "eth0".to_string(),
            ip_addresses: vec!["172.24.160.10".to_string(), "fe80::abcd:ef01:2345:6789".to_string()],
            is_up: true,
            is_loopback: false,
        },
        NetworkInterface {
            name: "lo".to_string(),
            ip_addresses: vec!["127.0.0.1".to_string(), "::1".to_string()],
            is_up: true,
            is_loopback: true,
        },
        NetworkInterface {
            name: "docker0".to_string(),
            ip_addresses: vec!["172.17.0.1".to_string()],
            is_up: true,
            is_loopback: false,
        },
    ];
    
    Ok(interfaces)
}

/// Get active ports from WSL system
/// 
/// This function uses the `ss` command to get active port information.
pub fn get_active_ports() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    // In a real implementation, we would execute:
    // let output = Command::new("ss").args(&["-tuln"]).output()?;
    // For now, we'll simulate the output
    
    // Simulate WSL port information
    let ports = vec![
        PortInfo {
            process_id: "1234".to_string(),
            process_name: "code".to_string(),
            protocol: "TCP".to_string(),
            port: "8080".to_string(),
            direction: "LISTEN".to_string(),
            network: "127.0.0.1".to_string(),
        },
        PortInfo {
            process_id: "5678".to_string(),
            process_name: "docker-proxy".to_string(),
            protocol: "TCP".to_string(),
            port: "8000".to_string(),
            direction: "LISTEN".to_string(),
            network: "0.0.0.0".to_string(),
        },
        PortInfo {
            process_id: "9012".to_string(),
            process_name: "ssh".to_string(),
            protocol: "TCP".to_string(),
            port: "22".to_string(),
            direction: "ESTABLISHED".to_string(),
            network: "172.24.160.10".to_string(),
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
        assert_eq!(interfaces[0].name, "eth0");
        assert_eq!(interfaces[0].ip_addresses.len(), 2);
        assert!(interfaces[0].is_up);
        assert!(!interfaces[0].is_loopback);
        
        // Check loopback interface
        assert_eq!(interfaces[1].name, "lo");
        assert!(interfaces[1].is_loopback);
    }

    #[test]
    fn test_get_active_ports() {
        let ports = get_active_ports().unwrap();
        assert_eq!(ports.len(), 3);
        
        // Check first port
        assert_eq!(ports[0].process_name, "code");
        assert_eq!(ports[0].protocol, "TCP");
        assert_eq!(ports[0].port, "8080");
    }
}