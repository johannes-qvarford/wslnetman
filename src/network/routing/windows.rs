//! Windows routing rules inspection implementation
//! 
//! This module provides functionality to inspect routing rules on Windows systems.
//! Since this application runs in WSL, we'll simulate Windows routing data for demonstration purposes.

use crate::network::Route;

/// Get routing rules from Windows system
/// 
/// In a real implementation, this would use Windows Routing API or parse the output
/// of `route print` command or `Get-NetRoute` PowerShell cmdlet.
pub fn get_routing_rules() -> Result<Vec<Route>, Box<dyn std::error::Error>> {
    // Simulate Windows routing rules
    let routes = vec![
        Route {
            destination: "0.0.0.0/0".to_string(),
            gateway: "192.168.1.1".to_string(),
            interface: "Ethernet".to_string(),
            metric: "25".to_string(),
        },
        Route {
            destination: "192.168.1.0/24".to_string(),
            gateway: "On-link".to_string(),
            interface: "Ethernet".to_string(),
            metric: "281".to_string(),
        },
        Route {
            destination: "192.168.1.1/32".to_string(),
            gateway: "On-link".to_string(),
            interface: "Ethernet".to_string(),
            metric: "281".to_string(),
        },
        Route {
            destination: "127.0.0.0/8".to_string(),
            gateway: "On-link".to_string(),
            interface: "Loopback".to_string(),
            metric: "331".to_string(),
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
        assert_eq!(routes[0].destination, "0.0.0.0/0");
        assert_eq!(routes[0].gateway, "192.168.1.1");
        assert_eq!(routes[0].interface, "Ethernet");
        assert_eq!(routes[0].metric, "25");
    }
}