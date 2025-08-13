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

/// Represents a Docker container with its properties
#[derive(Debug, Clone)]
pub struct DockerContainer {
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: String,
    pub id: String,
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

/// Get Docker containers for a specific network
///
/// This function uses the `docker ps` command to get containers connected to a specific network.
pub fn get_containers_for_network(
    network_name: &str,
) -> Result<Vec<DockerContainer>, Box<dyn std::error::Error>> {
    // Execute docker ps command with network filter and platform-specific handling
    let output = if cfg!(target_os = "windows") {
        // Windows: Use WSL to execute docker command
        Command::new("wsl.exe")
            .args([
                "-e",
                "docker",
                "ps",
                "--filter",
                &format!("network={network_name}"),
                "--format",
                "json",
            ])
            .output()
    } else {
        // WSL/Linux: Execute docker command directly
        Command::new("docker")
            .args([
                "ps",
                "--filter",
                &format!("network={network_name}"),
                "--format",
                "json",
            ])
            .output()
    };

    let mut containers = Vec::new();

    // Process output if available
    if let Ok(output) = output {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);

            // Parse the output to extract container information
            // Each line is a JSON object representing a container
            for line in output_str.lines() {
                if !line.trim().is_empty() {
                    // Try to parse each line as JSON
                    if let Ok(container_info) = serde_json::from_str::<serde_json::Value>(line) {
                        // Extract fields from the JSON object
                        let name = container_info
                            .get("Names")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        let image = container_info
                            .get("Image")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        let status = container_info
                            .get("Status")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        let ports = container_info
                            .get("Ports")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let id = container_info
                            .get("ID")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();

                        containers.push(DockerContainer {
                            name,
                            image,
                            status,
                            ports,
                            id,
                        });
                    }
                }
            }
        }
    } else if let Err(x) = output {
        println!("Docker container query error {x}");
    }

    Ok(containers)
}
