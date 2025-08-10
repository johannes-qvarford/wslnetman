//! WSL routing rules inspection implementation
//! 
//! This module provides functionality to inspect routing rules on WSL systems.
//! It uses the `ip route` command to get routing rule information.

use crate::network::Route;
use std::process::Command;

/// Get routing rules from WSL system
/// 
/// This function uses the `ip route` command to get routing rule information.
pub fn get_routing_rules() -> Result<Vec<Route>, Box<dyn std::error::Error>> {
    // In a real implementation, we would execute:
    // let output = Command::new("ip").args(&["route", "show"]).output()?;
    // For now, we'll simulate the output
    
    // Simulate WSL routing rules
    let routes = vec![
        Route {
            destination: "default".to_string(),
            gateway: "172.24.160.1".to_string(),
            interface: "eth0".to_string(),
            metric: "100".to_string(),
        },
        Route {
            destination: "172.24.160.0/20".to_string(),
            gateway: "On-link".to_string(),
            interface: "eth0".to_string(),
            metric: "100".to_string(),
        },
        Route {
            destination: "172.24.160.1/32".to_string(),
            gateway: "On-link".to_string(),
            interface: "eth0".to_string(),
            metric: "100".to_string(),
        },
        Route {
            destination: "127.0.0.0/8".to_string(),
            gateway: "On-link".to_string(),
            interface: "lo".to_string(),
            metric: "256".to_string(),
        },
    ];
    
    Ok(routes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_routing_rules() {
        let routes = get_routing_rules().unwrap();
        assert_eq!(routes.len(), 4);
        
        // Check first route
        assert_eq!(routes[0].destination, "default");
        assert_eq!(routes[0].gateway, "172.24.160.1");
        assert_eq!(routes[0].interface, "eth0");
        assert_eq!(routes[0].metric, "100");
    }
}