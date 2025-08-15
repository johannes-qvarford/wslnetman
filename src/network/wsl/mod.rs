//! WSL network discovery implementation
//!
//! This module provides functionality to discover network ports within the WSL environment.
//! It uses WSL-native commands executed via wsl.exe interop.

use crate::network::{NetworkEnvironment, PortInfo};
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

/// Get active ports from WSL system
///
/// This function uses `wsl.exe --user root` to get complete port and process information.
pub fn get_active_ports() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    // Use root access for complete process information
    match get_ports_with_root_access() {
        Ok(ports) => Ok(ports),
        Err(e) => {
            // Log error and return empty list
            log_error_to_file(&format!("WSL port discovery failed: {e}"));
            Ok(Vec::new())
        }
    }
}

/// Get ports using root access for complete process information
fn get_ports_with_root_access() -> Result<Vec<PortInfo>, Box<dyn std::error::Error>> {
    // Use root access to get ports with process information
    let ss_result = std::thread::spawn(|| {
        Command::new("wsl.exe")
            .args(["--user", "root", "-e", "timeout", "5", "ss", "-tulnp"])
            .output()
    })
    .join();

    match ss_result {
        Ok(Ok(ss_output)) => {
            if ss_output.status.success() {
                let output_str = String::from_utf8_lossy(&ss_output.stdout);
                let ports = parse_ss_with_process_info(&output_str);
                Ok(ports)
            } else {
                let stderr = String::from_utf8_lossy(&ss_output.stderr);
                let error_msg = format!(
                    "ss command failed with exit code {:?}. stderr: {}",
                    ss_output.status.code(),
                    stderr
                );
                log_error_to_file(&error_msg);
                Err(error_msg.into())
            }
        }
        Ok(Err(e)) => {
            let error_msg = format!("Failed to execute ss command: {e}");
            log_error_to_file(&error_msg);
            Err(error_msg.into())
        }
        Err(_) => {
            let error_msg = "ss command thread panicked or timed out".to_string();
            log_error_to_file(&error_msg);
            Err(error_msg.into())
        }
    }
}

/// Log error messages to a file for debugging
fn log_error_to_file(error_msg: &str) {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let log_entry = format!("[{timestamp}] {error_msg}\n");

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("wsl_port_discovery_errors.txt")
    {
        let _ = file.write_all(log_entry.as_bytes());
    }
}

/// Parse ss output with process information (from root ss -tulnp)
/// Format: "tcp   LISTEN 0   128   0.0.0.0:22   0.0.0.0:*   users:(("sshd",pid=1234,fd=3))"
fn parse_ss_with_process_info(output_str: &str) -> Vec<PortInfo> {
    let mut ports = Vec::new();

    for line in output_str.lines().skip(1) {
        // Skip header line
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 6 && parts[1] == "LISTEN" {
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

            // Extract process info from the users field (if present)
            let (process_id, process_name) = if parts.len() >= 7 {
                parse_users_field(parts[6])
            } else {
                ("N/A".to_string(), "N/A".to_string())
            };

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

/// Parse process information from ss users field
/// Format: "users:(("sshd",pid=1234,fd=3))"
fn parse_users_field(users_field: &str) -> (String, String) {
    if let Some(users_start) = users_field.find("users:((") {
        let users_part = &users_field[users_start + 8..];
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
    ("N/A".to_string(), "N/A".to_string())
}
