# Network Information Gathering

This module is responsible for collecting network information from both Windows and WSL environments.

## Approach

### Network Interface Discovery

1. **Windows**:
   - Use Windows API `GetAdaptersAddresses` to get detailed network interface information
   - Alternative: Parse output from `ipconfig` command

2. **WSL**:
   - Use `ip addr` command to get network interface information
   - Alternative: Read from `/proc/net/dev` for basic interface information

### Port Information

1. **Windows**:
   - Use Windows API `GetTcpTable`/`GetUdpTable` to get active connections
   - Alternative: Parse output from `netstat` command

2. **WSL**:
   - Use `ss` command (preferred) or `netstat` command to get active connections

### Implementation Plan

1. Create platform-specific modules for network discovery
2. Implement common interfaces that abstract platform differences
3. Handle errors gracefully and provide meaningful error messages
4. Cache results to avoid repeated expensive operations
5. Provide refresh functionality to update information