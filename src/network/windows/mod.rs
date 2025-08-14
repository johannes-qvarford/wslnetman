//! Windows network discovery implementation
//!
//! This module provides functionality to discover network interfaces and ports on Windows systems.
//! Since this application runs in WSL, we use WSL's interoperability with Windows to gather data.

use crate::network::{NetworkEnvironment, NetworkInterface, PortInfo};
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
    println!("WINDOWS: Getting network interfaces using PowerShell");
    // Try to get IP addresses from Windows
    let ip_output = Command::new("powershell.exe")
        .args([
            "-Command",
            "$OutputEncoding = [console]::InputEncoding = [console]::OutputEncoding = New-Object System.Text.UTF8Encoding; Get-NetIPAddress | Select-Object InterfaceAlias, IPAddress, AddressFamily | ConvertTo-Json -Depth 2"
        ])
        .output();

    // Try to get adapter information from Windows
    let adapter_output = Command::new("powershell.exe")
        .args([
            "-Command",
            "$OutputEncoding = [console]::InputEncoding = [console]::OutputEncoding = New-Object System.Text.UTF8Encoding; Get-NetAdapter | Select-Object Name, InterfaceDescription, ifIndex, Status, MacAddress | ConvertTo-Json -Depth 2"
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
            let adapter_str = String::from_utf8_lossy(&adapter_output.stdout);
            println!("WINDOWS: Adapter output:\n{adapter_str}");

            // Try to parse JSON output - handle both single object and array cases
            let adapter_result = serde_json::from_slice::<Vec<WindowsNetAdapter>>(
                &adapter_output.stdout,
            )
            .or_else(|_| {
                // If parsing as array fails, try parsing as single object
                serde_json::from_slice::<WindowsNetAdapter>(&adapter_output.stdout)
                    .map(|single| vec![single])
            });

            match adapter_result {
                Ok(adapters) => {
                    println!("WINDOWS: Parsed {} adapters", adapters.len());
                    for (idx, adapter) in adapters.iter().enumerate() {
                        println!(
                            "WINDOWS: Adapter {}: name='{}' status='{}' mac='{:?}'",
                            idx, adapter.name, adapter.status, adapter.mac_address
                        );
                        if let Some(mac) = &adapter.mac_address {
                            // Clean up MAC address format (remove dashes, add colons)
                            let clean_mac = mac.replace("-", ":");
                            adapter_map.insert(adapter.name.clone(), clean_mac);
                        }
                        adapter_status_map.insert(adapter.name.clone(), adapter.status == "Up");
                    }
                }
                Err(parse_error) => {
                    println!("WINDOWS: Failed to parse adapter JSON: {parse_error}");
                    println!(
                        "WINDOWS: Raw stdout length: {}",
                        adapter_output.stdout.len()
                    );
                    println!(
                        "WINDOWS: Raw stderr: {}",
                        String::from_utf8_lossy(&adapter_output.stderr)
                    );

                    // Try to identify problematic characters
                    let bytes = &adapter_output.stdout;
                    let mut non_ascii_positions = Vec::new();
                    for (i, &byte) in bytes.iter().enumerate() {
                        if byte > 127 {
                            non_ascii_positions.push((i, byte));
                        }
                    }
                    if !non_ascii_positions.is_empty() {
                        println!(
                            "WINDOWS: Found non-ASCII bytes at positions: {non_ascii_positions:?}"
                        );
                    }
                }
            }
        } else {
            println!("WINDOWS: Adapter PowerShell command failed");
        }
    } else if let Err(x) = adapter_output {
        println!("WINDOWS: Failed to get adapter output: {x}");
    }

    // Process IP address information if available
    if let Ok(ip_output) = ip_output {
        if ip_output.status.success() {
            let ip_str = String::from_utf8_lossy(&ip_output.stdout);
            println!("WINDOWS: IP output:\n{ip_str}");

            // Try to parse JSON output - handle both single object and array cases
            let ip_result = serde_json::from_slice::<Vec<WindowsIPAddress>>(&ip_output.stdout)
                .or_else(|_| {
                    // If parsing as array fails, try parsing as single object
                    serde_json::from_slice::<WindowsIPAddress>(&ip_output.stdout)
                        .map(|single| vec![single])
                });

            if let Ok(ip_addresses) = ip_result {
                println!("WINDOWS: Parsed {} IP addresses", ip_addresses.len());
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

                println!("{all_interfaces:?}");

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
    } else if let Err(x2) = ip_output {
        println!("Failed to get Windows ip output {x2}");
    }

    Ok(interfaces)
}

/// Get active ports from Windows system
///
/// This function uses PowerShell Get-NetTCPConnection and netstat to get active port information with process names.
pub fn get_active_ports() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    println!("WINDOWS: Getting active ports using PowerShell and netstat");

    // First try PowerShell Get-NetTCPConnection (Windows 8+)
    let ps_output = Command::new("powershell.exe")
        .args([
            "-Command",
            "Get-NetTCPConnection | Where-Object {$_.State -eq 'Listen'} | Select-Object LocalAddress, LocalPort, @{Name='ProcessName';Expression={(Get-Process -Id $_.OwningProcess -ErrorAction SilentlyContinue).Name}}, OwningProcess | ConvertTo-Json"
        ])
        .output();

    let mut ports = Vec::new();

    // Process PowerShell output if available
    if let Ok(ps_output) = ps_output {
        if ps_output.status.success() {
            let output_str = String::from_utf8_lossy(&ps_output.stdout);
            println!("WINDOWS: PowerShell output:\n{output_str}");

            // Parse JSON output
            if let Ok(connections) = parse_powershell_connections(&output_str) {
                ports.extend(connections);
            }
        } else {
            println!("WINDOWS: PowerShell command failed, falling back to netstat");
        }
    }

    // Fallback to netstat if PowerShell failed or returned no results
    if ports.is_empty() {
        println!("WINDOWS: Using netstat fallback");

        let netstat_output = Command::new("netstat").args(["-ano", "-p", "TCP"]).output();

        if let Ok(netstat_output) = netstat_output {
            if netstat_output.status.success() {
                let output_str = String::from_utf8_lossy(&netstat_output.stdout);
                println!("WINDOWS: netstat output:\n{output_str}");

                ports.extend(parse_netstat_output(&output_str));
            }
        }
    }

    println!("WINDOWS: Retrieved {} total ports", ports.len());
    Ok(ports)
}

/// Parse PowerShell Get-NetTCPConnection JSON output
fn parse_powershell_connections(
    json_str: &str,
) -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    #[derive(Deserialize, Debug)]
    struct PSConnection {
        #[serde(rename = "LocalAddress")]
        local_address: String,
        #[serde(rename = "LocalPort")]
        local_port: u16,
        #[serde(rename = "ProcessName")]
        process_name: Option<String>,
        #[serde(rename = "OwningProcess")]
        owning_process: Option<u32>,
    }

    let mut ports = Vec::new();

    // Handle both single object and array cases
    let connections: Vec<PSConnection> = serde_json::from_str(json_str)
        .or_else(|_| serde_json::from_str::<PSConnection>(json_str).map(|single| vec![single]))?;

    for (idx, conn) in connections.iter().enumerate() {
        let process_name = conn
            .process_name
            .clone()
            .unwrap_or_else(|| "N/A".to_string());

        let process_id = conn
            .owning_process
            .map(|pid| pid.to_string())
            .unwrap_or_else(|| "N/A".to_string());

        let port_info = PortInfo {
            process_id,
            process_name: process_name.clone(),
            protocol: "TCP".to_string(),
            port: conn.local_port.to_string(),
            direction: "LISTEN".to_string(),
            network: format!("{}:{}", conn.local_address, conn.local_port),
        };

        println!(
            "WINDOWS: PowerShell {} - TCP:{} process='{}' on {}",
            idx, conn.local_port, process_name, conn.local_address
        );

        ports.push(port_info);
    }

    Ok(ports)
}

/// Parse netstat output as fallback
fn parse_netstat_output(output_str: &str) -> Vec<PortInfo> {
    let mut ports = Vec::new();

    for (line_num, line) in output_str.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // netstat -ano format: Proto Local_Address Foreign_Address State PID
        if parts.len() >= 5 && parts[0] == "TCP" && parts[3] == "LISTENING" {
            let local_address = parts[1];
            let pid = parts[4];

            // Extract port from address:port
            let port = if let Some(colon_pos) = local_address.rfind(':') {
                &local_address[colon_pos + 1..]
            } else {
                local_address
            };

            let port_info = PortInfo {
                process_id: pid.to_string(),
                process_name: get_process_name_by_pid(pid),
                protocol: "TCP".to_string(),
                port: port.to_string(),
                direction: "LISTENING".to_string(),
                network: local_address.to_string(),
            };

            println!("WINDOWS: netstat {line_num} - TCP:{port} PID={pid} on {local_address}");

            ports.push(port_info);
        }
    }

    ports
}

/// Get process name by PID using tasklist
fn get_process_name_by_pid(pid: &str) -> String {
    let tasklist_output = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {pid}"), "/FO", "CSV", "/NH"])
        .output();

    if let Ok(output) = tasklist_output {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Parse CSV format: "ProcessName","PID","SessionName","Session#","MemUsage"
            if let Some(line) = output_str.lines().next() {
                if let Some(first_quote_end) = line[1..].find('"') {
                    return line[1..first_quote_end + 1].to_string();
                }
            }
        }
    }

    "N/A".to_string()
}
