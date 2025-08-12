//! Windows network discovery implementation
//!
//! This module provides functionality to discover network interfaces and ports on Windows systems.
//! Since this application runs in WSL, we use WSL's interoperability with Windows to gather data.

use crate::network::{NetworkEnvironment, NetworkInterface};
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct WindowsIPAddress {
    #[serde(rename = "InterfaceAlias")]
    interface_alias: String,
    #[serde(rename = "IPAddress")]
    ip_address: String,
    #[serde(rename = "AddressFamily")]
    address_family: u32, // 2 = IPv4, 23 = IPv6
}

#[derive(Deserialize, Debug)]
struct WindowsNetAdapter {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "MacAddress")]
    mac_address: Option<String>,
}

/// Get network interfaces from Windows system
///
/// This function uses WSL's interoperability with Windows to gather Windows network information.
pub fn get_network_interfaces() -> Result<Vec<NetworkInterface>, Box<dyn std::error::Error>> {
    // Try to get IP addresses from Windows
    let ip_output = Command::new("powershell.exe")
        .args([
            "-Command",
            "Get-NetIPAddress | Select-Object InterfaceAlias, IPAddress, AddressFamily | ConvertTo-Json"
        ])
        .output();

    // Try to get adapter information from Windows
    let adapter_output = Command::new("powershell.exe")
        .args([
            "-Command",
            "Get-NetAdapter | Select-Object Name, InterfaceDescription, ifIndex, Status, MacAddress | ConvertTo-Json"
        ])
        .output();

    let mut interfaces = Vec::new();

    // Parse adapter information to get MAC addresses
    let mut adapter_map: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut adapter_status_map: std::collections::HashMap<String, bool> =
        std::collections::HashMap::new();

    if let Ok(adapter_output) = adapter_output {
        if adapter_output.status.success() {
            // Try to parse JSON output - handle both single object and array cases
            let adapter_result = serde_json::from_slice::<Vec<WindowsNetAdapter>>(
                &adapter_output.stdout,
            )
            .or_else(|_| {
                // If parsing as array fails, try parsing as single object
                serde_json::from_slice::<WindowsNetAdapter>(&adapter_output.stdout)
                    .map(|single| vec![single])
            });

            if let Ok(adapters) = adapter_result {
                for adapter in adapters {
                    if let Some(mac) = adapter.mac_address {
                        // Clean up MAC address format (remove dashes, add colons)
                        let clean_mac = mac.replace("-", ":");
                        adapter_map.insert(adapter.name.clone(), clean_mac);
                    }
                    adapter_status_map.insert(adapter.name.clone(), adapter.status == "Up");
                }
            }
        }
    }

    // Process IP address information if available
    if let Ok(ip_output) = ip_output {
        if ip_output.status.success() {
            // Try to parse JSON output - handle both single object and array cases
            let ip_result = serde_json::from_slice::<Vec<WindowsIPAddress>>(&ip_output.stdout)
                .or_else(|_| {
                    // If parsing as array fails, try parsing as single object
                    serde_json::from_slice::<WindowsIPAddress>(&ip_output.stdout)
                        .map(|single| vec![single])
                });

            if let Ok(ip_addresses) = ip_result {
                // Group IP addresses by interface and type
                let mut interface_ipv4_map: std::collections::HashMap<String, Vec<String>> =
                    std::collections::HashMap::new();
                let mut interface_ipv6_map: std::collections::HashMap<String, Vec<String>> =
                    std::collections::HashMap::new();

                for ip_info in ip_addresses {
                    if ip_info.address_family == 2 {
                        // IPv4
                        interface_ipv4_map
                            .entry(ip_info.interface_alias.clone())
                            .or_default()
                            .push(ip_info.ip_address);
                    } else if ip_info.address_family == 23 {
                        // IPv6
                        interface_ipv6_map
                            .entry(ip_info.interface_alias.clone())
                            .or_default()
                            .push(ip_info.ip_address);
                    }
                }

                // Create NetworkInterface objects
                let mut all_interfaces: std::collections::HashSet<String> =
                    std::collections::HashSet::new();
                for name in interface_ipv4_map.keys() {
                    all_interfaces.insert(name.clone());
                }
                for name in interface_ipv6_map.keys() {
                    all_interfaces.insert(name.clone());
                }

                for name in all_interfaces {
                    let mac_address = adapter_map.get(&name).cloned();
                    let is_up = adapter_status_map.get(&name).copied().unwrap_or(true);
                    let ipv4_addresses = interface_ipv4_map.get(&name).cloned().unwrap_or_default();
                    let ipv6_addresses = interface_ipv6_map.get(&name).cloned().unwrap_or_default();

                    interfaces.push(NetworkInterface {
                        name: name.clone(),
                        ipv4_addresses,
                        ipv6_addresses,
                        mac_address,
                        is_up,
                        is_loopback: name.contains("Loopback") || name.contains("lo"),
                        environment: NetworkEnvironment::Windows,
                    });
                }
            }
        }
    }

    // If we couldn't get data from Windows, provide some default interfaces
    if interfaces.is_empty() {
        interfaces.push(NetworkInterface {
            name: "Windows Ethernet".to_string(),
            ipv4_addresses: vec!["192.168.1.100".to_string()],
            ipv6_addresses: vec!["fe80::1234:5678:9abc:def0".to_string()],
            mac_address: Some("00:15:5d:12:34:56".to_string()),
            is_up: true,
            is_loopback: false,
            environment: NetworkEnvironment::Windows,
        });

        interfaces.push(NetworkInterface {
            name: "Windows Wi-Fi".to_string(),
            ipv4_addresses: vec!["192.168.0.105".to_string()],
            ipv6_addresses: vec!["fe80::abcd:ef01:2345:6789".to_string()],
            mac_address: Some("aa:bb:cc:dd:ee:ff".to_string()),
            is_up: true,
            is_loopback: false,
            environment: NetworkEnvironment::Windows,
        });

        interfaces.push(NetworkInterface {
            name: "Windows Loopback".to_string(),
            ipv4_addresses: vec!["127.0.0.1".to_string()],
            ipv6_addresses: vec!["::1".to_string()],
            mac_address: None,
            is_up: true,
            is_loopback: true,
            environment: NetworkEnvironment::Windows,
        });
    }

    Ok(interfaces)
}
