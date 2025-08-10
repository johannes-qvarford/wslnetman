//! Routing module
//! 
//! This module provides functionality to inspect routing rules on both Windows and WSL systems.

pub mod windows;
pub mod wsl;

/// Represents a routing rule with its properties
#[derive(Debug, Clone)]
pub struct Route {
    pub destination: String,
    pub gateway: String,
    pub interface: String,
    pub metric: String,
}

/// Get routing rules from the current system
/// 
/// This function returns routing rules from either Windows or WSL
/// depending on the compilation target.
pub fn get_routing_rules() -> Result<Vec<Route>, Box<dyn std::error::Error>> {
    // For demonstration purposes, we'll use WSL routing rules
    // In a real implementation, we would detect the platform and call the appropriate function
    wsl::get_routing_rules()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_routing_rules() {
        let rules = get_routing_rules().unwrap();
        assert!(!rules.is_empty());
    }
}