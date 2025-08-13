//! Docker network discovery implementation
//!
//! This module provides functionality to discover Docker networks.
//! It uses the `docker network ls` command to get network information.

use std::process::Command;

/// Represents a Docker network with its properties and source environment
#[derive(Debug, Clone)]
pub struct DockerNetwork {
    pub name: String,
    pub driver: String,
    pub scope: String,
    pub subnet: String,
}

/// Get Docker networks
///
/// This function uses the `docker network ls` command to get Docker network information.
/// In a real implementation, we would also parse `docker network inspect` for detailed information.
pub fn get_docker_networks() -> Result<Vec<DockerNetwork>, Box<dyn std::error::Error>> {
    // Execute docker network ls command with platform-specific handling
    let output = if cfg!(target_os = "windows") {
        // Windows: Use WSL to execute docker command
        Command::new("wsl.exe")
            .args(["-e", "docker", "network", "ls", "--format", "json"])
            .output()
    } else {
        // WSL/Linux: Execute docker command directly
        Command::new("docker")
            .args(["network", "ls", "--format", "json"])
            .output()
    };

    let mut networks = Vec::new();

    // Process output if available
    if let Ok(output) = output {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);

            // Parse the output to extract network information
            // Each line is a JSON object representing a network
            for line in output_str.lines() {
                if !line.trim().is_empty() {
                    // Try to parse each line as JSON
                    if let Ok(network_info) = serde_json::from_str::<serde_json::Value>(line) {
                        // Extract fields from the JSON object
                        let name = network_info
                            .get("Name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        let driver = network_info
                            .get("Driver")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        let scope = network_info
                            .get("Scope")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();

                        // For subnet information, we would need to run docker network inspect
                        // For now, we'll leave it empty
                        let subnet = "".to_string();

                        networks.push(DockerNetwork {
                            name,
                            driver,
                            scope,
                            subnet,
                        });
                    }
                }
            }
        }
    } else if let Err(x) = output {
        println!("Docker network error {x}");
    }

    // If we couldn't get data, provide some default networks
    if networks.is_empty() {
        println!("Could not load any networks for Docker!");
    }

    Ok(networks)
}
