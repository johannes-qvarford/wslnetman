//! WSL network discovery implementation
//!
//! This module provides functionality to discover network ports within the WSL environment.
//! It uses WSL-native commands executed via wsl.exe interop.

use crate::network::{NetworkEnvironment, PortInfo};
use std::process::Command;

/// Get active ports from WSL system
///
/// This function uses `wsl.exe` to execute WSL-native commands to get active port information.
pub fn get_active_ports() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    // Try using ss command first (more modern and provides better info)
    let ss_output = Command::new("wsl.exe").args(["-e", "ss", "-tuln"]).output();

    let mut ports = Vec::new();

    // Process ss output if available
    if let Ok(ss_output) = ss_output {
        if ss_output.status.success() {
            let output_str = String::from_utf8_lossy(&ss_output.stdout);
            ports.extend(parse_ss_output(&output_str));
        }
    }

    // Fallback to netstat if ss failed or returned no results
    if ports.is_empty() {
        let netstat_output = Command::new("wsl.exe")
            .args(["-e", "netstat", "-tuln"])
            .output();

        if let Ok(netstat_output) = netstat_output {
            if netstat_output.status.success() {
                let output_str = String::from_utf8_lossy(&netstat_output.stdout);
                ports.extend(parse_wsl_netstat_output(&output_str));
            }
        }
    }

    Ok(ports)
}

/// Parse ss command output
/// Format: "tcp   LISTEN 0      128          0.0.0.0:22       0.0.0.0:*"
fn parse_ss_output(output_str: &str) -> Vec<PortInfo> {
    let mut ports = Vec::new();

    for line in output_str.lines().skip(1) {
        // Skip header line
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 5 && parts[1] == "LISTEN" {
            let protocol = parts[0].to_uppercase();
            let local_address = parts[4];

            // Extract port from address:port
            let port = if let Some(colon_pos) = local_address.rfind(':') {
                &local_address[colon_pos + 1..]
            } else {
                local_address
            };

            // Skip if port parsing failed or is not numeric
            if port.parse::<u16>().is_err() {
                continue;
            }

            // Get process info for this port
            let (process_id, process_name) = get_process_info_for_port(&protocol, port);

            let port_info = PortInfo {
                process_id,
                process_name,
                protocol,
                port: port.to_string(),
                direction: "LISTEN".to_string(),
                network: local_address.to_string(),
                environment: NetworkEnvironment::Wsl,
            };

            ports.push(port_info);
        }
    }

    ports
}

/// Parse WSL netstat output as fallback
/// Format: "tcp        0      0 0.0.0.0:22              0.0.0.0:*               LISTEN"
fn parse_wsl_netstat_output(output_str: &str) -> Vec<PortInfo> {
    let mut ports = Vec::new();

    for line in output_str.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // netstat -tuln format: Proto Recv-Q Send-Q Local_Address Foreign_Address State
        if parts.len() >= 6 && (parts[0] == "tcp" || parts[0] == "udp") && parts[5] == "LISTEN" {
            let protocol = parts[0].to_uppercase();
            let local_address = parts[3];

            // Extract port from address:port
            let port = if let Some(colon_pos) = local_address.rfind(':') {
                &local_address[colon_pos + 1..]
            } else {
                local_address
            };

            // Skip if port parsing failed or is not numeric
            if port.parse::<u16>().is_err() {
                continue;
            }

            // Get process info for this port
            let (process_id, process_name) = get_process_info_for_port(&protocol, port);

            let port_info = PortInfo {
                process_id,
                process_name,
                protocol,
                port: port.to_string(),
                direction: "LISTEN".to_string(),
                network: local_address.to_string(),
                environment: NetworkEnvironment::Wsl,
            };

            ports.push(port_info);
        }
    }

    ports
}

/// Get process information for a specific port using lsof or ss with process info
fn get_process_info_for_port(protocol: &str, port: &str) -> (String, String) {
    // Try using ss with process info first
    let ss_process_output = Command::new("wsl.exe")
        .args(["-e", "ss", "-tulnp", &format!("sport = :{port}")])
        .output();

    if let Ok(ss_output) = ss_process_output {
        if ss_output.status.success() {
            let output_str = String::from_utf8_lossy(&ss_output.stdout);

            // Parse ss output with process info
            // Format: "tcp   LISTEN 0   128   0.0.0.0:22   0.0.0.0:*   users:(("sshd",pid=1234,fd=3))"
            for line in output_str.lines().skip(1) {
                if line.contains(&format!(":{port}")) {
                    if let Some(users_start) = line.find("users:((") {
                        let users_part = &line[users_start + 8..];
                        if let Some(comma_pos) = users_part.find(',') {
                            let process_name = &users_part[1..comma_pos - 1]; // Remove quotes

                            // Extract PID
                            if let Some(pid_start) = users_part.find("pid=") {
                                let pid_part = &users_part[pid_start + 4..];
                                if let Some(pid_end) = pid_part.find(',') {
                                    let pid = &pid_part[..pid_end];
                                    return (pid.to_string(), process_name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Fallback to lsof if ss with process info failed
    let lsof_output = Command::new("wsl.exe")
        .args([
            "-e",
            "lsof",
            "-i",
            &format!("{}:{}", protocol.to_lowercase(), port),
            "-t", // Only show PIDs
        ])
        .output();

    if let Ok(lsof_output) = lsof_output {
        if lsof_output.status.success() {
            let output_str = String::from_utf8_lossy(&lsof_output.stdout);
            if let Some(line) = output_str.lines().next() {
                if let Ok(pid) = line.trim().parse::<u32>() {
                    // Get process name from PID
                    let process_name = get_process_name_by_pid(pid);
                    return (pid.to_string(), process_name);
                }
            }
        }
    }

    // Default if both methods fail
    ("N/A".to_string(), "N/A".to_string())
}

/// Get process name by PID using ps command
fn get_process_name_by_pid(pid: u32) -> String {
    let ps_output = Command::new("wsl.exe")
        .args(["-e", "ps", "-p", &pid.to_string(), "-o", "comm="])
        .output();

    if let Ok(output) = ps_output {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Some(process_name) = output_str.lines().next() {
                return process_name.trim().to_string();
            }
        }
    }

    "N/A".to_string()
}
