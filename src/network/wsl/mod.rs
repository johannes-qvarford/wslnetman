//! WSL network discovery implementation
//!
//! This module provides functionality to discover network interfaces and ports on WSL systems.
//! It uses command-line tools available in Linux to gather network information.

use crate::network::{NetworkEnvironment, NetworkInterface, PortInfo};
use std::process::Command;

/// Get network interfaces from WSL system
///
/// This function uses the `ip addr` and `ip link` commands to get network interface information.
pub fn get_network_interfaces() -> Result<Vec<NetworkInterface>, Box<dyn std::error::Error>> {
    // Execute ip -br addr show command for brief format
    let addr_output = Command::new("ip").args(["-br", "addr", "show"]).output();

    // Execute ip -br link show command to get MAC addresses in brief format
    let link_output = Command::new("ip").args(["-br", "link", "show"]).output();

    let mut interfaces = Vec::new();

    // Parse MAC addresses from ip -br link output
    let mut mac_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    if let Ok(link_output) = link_output {
        if link_output.status.success() {
            let link_str = String::from_utf8_lossy(&link_output.stdout);
            parse_brief_mac_addresses(&link_str, &mut mac_map);
        }
    }

    // Process addr output if available
    if let Ok(addr_output) = addr_output {
        if addr_output.status.success() {
            let output_str = String::from_utf8_lossy(&addr_output.stdout);

            // Parse the brief format output - each line is one interface
            // Format: "eth0 UP 172.20.11.89/20 fe80::215:5dff:fef9:e225/64"
            for line in output_str.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                if let Some(interface) = parse_brief_addr_line(trimmed, &mac_map) {
                    interfaces.push(interface);
                }
            }
        }
    } else if let Err(x) = addr_output {
        println!("Failed to parse addr output: {x}");
    }

    Ok(interfaces)
}

/// Parse a brief format address line
/// Format: "eth0 UP 172.20.11.89/20 fe80::215:5dff:fef9:e225/64"
fn parse_brief_addr_line(
    line: &str,
    mac_map: &std::collections::HashMap<String, String>,
) -> Option<NetworkInterface> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    let interface_name = parts[0].to_string();
    let state = parts[1];
    let is_up = state == "UP";
    let is_loopback = interface_name.starts_with("lo");
    let mac_address = mac_map.get(&interface_name).cloned();

    let mut ipv4_addresses = Vec::new();
    let mut ipv6_addresses = Vec::new();

    // Parse IP addresses from the remaining parts (index 2 onwards)
    for part in &parts[2..] {
        if let Some((ip_address, is_ipv6)) = parse_ip_address_with_type(part) {
            if is_ipv6 {
                ipv6_addresses.push(ip_address);
            } else {
                ipv4_addresses.push(ip_address);
            }
        }
    }

    Some(NetworkInterface {
        name: interface_name,
        ipv4_addresses,
        ipv6_addresses,
        mac_address,
        is_up,
        is_loopback,
        environment: NetworkEnvironment::Wsl,
    })
}

/// Parse an IP address and determine if it's IPv6
fn parse_ip_address_with_type(addr_with_prefix: &str) -> Option<(String, bool)> {
    // Extract IP address without the subnet mask
    let ip_address = if let Some(slash_pos) = addr_with_prefix.find('/') {
        &addr_with_prefix[..slash_pos]
    } else {
        addr_with_prefix
    };

    // Determine if it's IPv6 (contains colons) or IPv4
    let is_ipv6 = ip_address.contains(':');
    Some((ip_address.to_string(), is_ipv6))
}

/// Get active ports from WSL system
///
/// This function uses the `ss` command to get active port information.
pub fn get_active_ports() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    println!("WSL: Getting active ports using 'ss -tuln'");

    // Execute ss command to get listening ports
    let output = Command::new("ss").args(["-tuln"]).output();

    let mut ports = Vec::new();

    // Process output if available
    if let Ok(output) = output {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            println!("WSL: ss command output:\n{output_str}");

            // Parse the output to extract port information
            // Example line: "tcp    LISTEN  0      128          0.0.0.0:8080              0.0.0.0:*"
            for (line_num, line) in output_str.lines().enumerate() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 && (parts[0] == "tcp" || parts[0] == "udp") {
                    let port_info = PortInfo {
                        process_id: "N/A".to_string(), // ss -tuln doesn't show process info
                        process_name: "N/A".to_string(),
                        protocol: parts[0].to_uppercase(),
                        port: extract_port(parts[4]).to_string(),
                        direction: parts[1].to_string(),
                        network: parts[3].to_string(),
                    };
                    println!(
                        "WSL: Line {}: Parsed port {} on {} ({})",
                        line_num, port_info.port, port_info.network, port_info.protocol
                    );
                    ports.push(port_info);
                }
            }
        }
    } else if let Err(x) = output {
        println!("Error getting active ports: {x}");
    }

    Ok(ports)
}

/// Extract port number from address:port string
fn extract_port(address_port: &str) -> &str {
    if let Some(colon_pos) = address_port.rfind(":") {
        &address_port[colon_pos + 1..]
    } else {
        address_port
    }
}

/// Parse MAC addresses from ip -br link show output
/// Format: "eth0 UP 00:15:5d:f9:e2:25 <BROADCAST,MULTICAST,UP,LOWER_UP>"
fn parse_brief_mac_addresses(
    link_output: &str,
    mac_map: &mut std::collections::HashMap<String, String>,
) {
    for line in link_output.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 3 {
            let interface_name = parts[0].to_string();
            let mac_address = parts[2];

            // Check if this looks like a MAC address (contains colons)
            if mac_address.contains(':') && mac_address.len() == 17 {
                mac_map.insert(interface_name, mac_address.to_string());
            }
        }
    }
}
