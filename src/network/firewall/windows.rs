//! Windows firewall rules inspection implementation
//! 
//! This module provides functionality to inspect firewall rules on Windows systems.
//! Since this application runs in WSL, we'll simulate Windows firewall data for demonstration purposes.

use crate::network::FirewallRule;

/// Get firewall rules from Windows system
/// 
/// In a real implementation, this would use Windows Firewall API or parse the output
/// of `netsh advfirewall firewall show rule name=all` command.
pub fn get_firewall_rules() -> Result<Vec<FirewallRule>, Box<dyn std::error::Error>> {
    // Simulate Windows firewall rules
    let rules = vec![
        FirewallRule {
            name: "Windows Defender Firewall".to_string(),
            enabled: "Yes".to_string(),
            direction: "Inbound".to_string(),
            action: "Allow".to_string(),
            protocol: "TCP".to_string(),
            local_address: "192.168.1.100".to_string(),
            remote_address: "Any".to_string(),
        },
        FirewallRule {
            name: "Remote Desktop".to_string(),
            enabled: "No".to_string(),
            direction: "Inbound".to_string(),
            action: "Allow".to_string(),
            protocol: "TCP".to_string(),
            local_address: "192.168.1.100".to_string(),
            remote_address: "Any".to_string(),
        },
        FirewallRule {
            name: "File and Printer Sharing".to_string(),
            enabled: "Yes".to_string(),
            direction: "Outbound".to_string(),
            action: "Allow".to_string(),
            protocol: "UDP".to_string(),
            local_address: "192.168.1.100".to_string(),
            remote_address: "192.168.1.0/24".to_string(),
        },
        FirewallRule {
            name: "Block Malicious Traffic".to_string(),
            enabled: "Yes".to_string(),
            direction: "Inbound".to_string(),
            action: "Block".to_string(),
            protocol: "Any".to_string(),
            local_address: "Any".to_string(),
            remote_address: "10.0.0.0/8".to_string(),
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
        assert_eq!(rules[0].name, "Windows Defender Firewall");
        assert_eq!(rules[0].enabled, "Yes");
        assert_eq!(rules[0].direction, "Inbound");
        assert_eq!(rules[0].action, "Allow");
    }
}