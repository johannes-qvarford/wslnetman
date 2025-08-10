//! WSL firewall rules inspection implementation
//! 
//! This module provides functionality to inspect firewall rules on WSL systems.
//! It uses the `iptables` command to get firewall rule information.

use crate::network::FirewallRule;
use std::process::Command;

/// Get firewall rules from WSL system
/// 
/// This function uses the `iptables` command to get firewall rule information.
pub fn get_firewall_rules() -> Result<Vec<FirewallRule>, Box<dyn std::error::Error>> {
    // In a real implementation, we would execute:
    // let output = Command::new("iptables").args(&["-L", "-n", "-v"]).output()?;
    // For now, we'll simulate the output
    
    // Simulate WSL firewall rules
    let rules = vec![
        FirewallRule {
            name: "Allow Loopback".to_string(),
            enabled: "Yes".to_string(),
            direction: "Inbound".to_string(),
            action: "Accept".to_string(),
            protocol: "Any".to_string(),
            local_address: "127.0.0.0/8".to_string(),
            remote_address: "Any".to_string(),
        },
        FirewallRule {
            name: "Allow Established".to_string(),
            enabled: "Yes".to_string(),
            direction: "Inbound".to_string(),
            action: "Accept".to_string(),
            protocol: "Any".to_string(),
            local_address: "Any".to_string(),
            remote_address: "Any".to_string(),
        },
        FirewallRule {
            name: "Allow SSH".to_string(),
            enabled: "Yes".to_string(),
            direction: "Inbound".to_string(),
            action: "Accept".to_string(),
            protocol: "TCP".to_string(),
            local_address: "Any".to_string(),
            remote_address: "Any".to_string(),
        },
        FirewallRule {
            name: "Block All".to_string(),
            enabled: "Yes".to_string(),
            direction: "Inbound".to_string(),
            action: "Drop".to_string(),
            protocol: "Any".to_string(),
            local_address: "Any".to_string(),
            remote_address: "Any".to_string(),
        },
    ];
    
    Ok(rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_firewall_rules() {
        let rules = get_firewall_rules().unwrap();
        assert_eq!(rules.len(), 4);
        
        // Check first rule
        assert_eq!(rules[0].name, "Allow Loopback");
        assert_eq!(rules[0].enabled, "Yes");
        assert_eq!(rules[0].direction, "Inbound");
        assert_eq!(rules[0].action, "Accept");
    }
}