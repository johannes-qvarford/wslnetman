# Routing Rules Inspection

This module is responsible for inspecting routing rules on both Windows and WSL systems.

## Approach

### Windows Routing

1. **Primary Method**: Use Windows Routing API (if available through Rust bindings)
2. **Alternative Method**: Parse output from `route print` command or `Get-NetRoute` PowerShell cmdlet
3. **Data Format**: Extract destination, gateway, interface, and metric information

### WSL Routing (ip route)

1. **Primary Method**: Use `ip route` command
2. **Commands**:
   - `ip route show` for listing routing rules
   - `ip -6 route show` for IPv6 routing rules
3. **Data Format**: Extract destination, gateway, interface, and metric information

### Implementation Plan

1. Create platform-specific modules for routing inspection
2. Define common data structures for routing rules
3. Implement parsing logic for command output
4. Handle errors gracefully and provide meaningful error messages
5. Cache results to avoid repeated expensive operations
6. Provide refresh functionality to update information
7. Support both IPv4 and IPv6 routing rules