# Firewall Rules Inspection

This module is responsible for inspecting firewall rules on both Windows and WSL systems.

## Approach

### Windows Firewall

1. **Primary Method**: Use Windows Firewall API (if available through Rust bindings)
2. **Alternative Method**: Parse output from `netsh advfirewall firewall show rule name=all` command
3. **Data Format**: Extract rule name, enabled status, direction, action, protocol, local/remote addresses

### WSL Firewall (iptables)

1. **Primary Method**: Use `iptables` command with appropriate flags
2. **Commands**:
   - `iptables -L -n -v` for listing rules with numeric output and packet/byte counters
   - `iptables -t nat -L -n -v` for NAT table rules
3. **Data Format**: Extract chain, target, protocol, options, input/output interfaces, source/destination

### Implementation Plan

1. Create platform-specific modules for firewall inspection
2. Define common data structures for firewall rules
3. Implement parsing logic for command output
4. Handle errors gracefully and provide meaningful error messages
5. Cache results to avoid repeated expensive operations
6. Provide refresh functionality to update information