//! Firewall module
//! 
//! This module provides functionality to inspect firewall rules on both Windows and WSL systems.

pub mod windows;
pub mod wsl;

/// Represents a firewall rule with its properties
#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub name: String,
    pub enabled: String,
    pub direction: String,
    pub action: String,
    pub protocol: String,
    pub local_address: String,
    pub remote_address: String,
}

/// Get firewall rules from the current system
/// 
/// This function returns firewall rules from either Windows or WSL
/// depending on the compilation target.
pub fn get_firewall_rules() -> Result<Vec<FirewallRule>, Box<dyn std::error::Error>> {
    // For demonstration purposes, we'll use WSL firewall rules
    // In a real implementation, we would detect the platform and call the appropriate function
    wsl::get_firewall_rules()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_firewall_rules() {
        let rules = get_firewall_rules().unwrap();
        assert!(!rules.is_empty());
    }
}