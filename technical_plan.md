# WSLNetMan Technical Implementation Plan

## Dependencies

### Cargo.toml Dependencies

```toml
[dependencies]
slint = "1.0"
tokio = { version = "1", features = ["full"] }
reqwest = "0.11"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio-util = { version = "0.7", features = ["codec"] }
futures = "0.3"
clap = { version = "4.0", features = ["derive"] }

[target.'cfg(windows)'.dependencies]
windows = { version = "0.52", features = [
    "Win32_NetworkManagement_IpHelper",
    "Win32_Foundation",
    "Win32_System_Threading",
    "Win32_Networking_WinSock",
] }

[target.'cfg(unix)'.dependencies]
nix = "0.27"
socket2 = "0.5"
```

## Network Information Gathering

### Windows Implementation

1. **Network Interfaces**:
   - Use `GetAdaptersInfo` or `GetAdaptersAddresses` from IpHelper API
   - Parse adapter information including IP addresses, subnet masks, etc.

2. **Firewall Rules**:
   - Use `netsh advfirewall` commands or Windows Firewall APIs
   - Parse output to extract rule information

3. **Routing Table**:
   - Use `GetIpForwardTable` from IpHelper API
   - Alternative: Parse `route print` command output

4. **Docker Networks**:
   - Execute `docker network ls` and `docker network inspect` commands
   - Parse JSON output for network information

### WSL Implementation

1. **Network Interfaces**:
   - Execute `ip addr show` command
   - Parse output to extract interface information

2. **Firewall Rules**:
   - Execute `sudo iptables -L -n -v` command
   - Parse output to extract rule information

3. **Routing Table**:
   - Execute `ip route show` command
   - Parse output to extract routing information

4. **Docker Networks**:
   - Execute `docker network ls` and `docker network inspect` commands
   - Parse JSON output for network information

## Packet Sending Implementation

### Ping Functionality

1. **Windows**:
   - Use ICMP API or execute system ping command
   - Handle ICMP socket creation and packet sending

2. **WSL**:
   - Use ICMP sockets or execute system ping command
   - Handle raw socket permissions

### HTTP over TCP

1. **Implementation**:
   - Use `reqwest` crate for HTTP requests
   - Allow specification of source IP/interface

2. **Network Interface Selection**:
   - Use `socket2` crate to create sockets with specific bindings
   - Bind to specific local IP addresses before connecting

## Data Structures

### Network Information

```rust
struct NetworkInterface {
    name: String,
    ip_addresses: Vec<IpAddr>,
    mac_address: Option<String>,
    is_up: bool,
    is_loopback: bool,
}

struct PortInfo {
    process_id: u32,
    process_name: String,
    protocol: String, // TCP/UDP
    port: u16,
    direction: String, // Incoming/Outgoing
    network: String,
}

struct FirewallRule {
    name: String,
    enabled: bool,
    direction: String, // Inbound/Outbound
    action: String,    // Allow/Block
    protocol: String,
    local_address: String,
    remote_address: String,
    local_port: String,
    remote_port: String,
}

struct Route {
    destination: String,
    gateway: String,
    interface: String,
    metric: u32,
}

struct DockerNetwork {
    id: String,
    name: String,
    driver: String,
    scope: String,
    ipam: IpamConfig,
    containers: Vec<ContainerInfo>,
}

struct IpamConfig {
    driver: String,
    subnet: String,
    gateway: String,
}
```

## UI Components

### Main Window
- Tabbed interface for different inspection tools
- Status bar showing current operations

### Network Widget
- Tree view showing networks from Windows, WSL, and Docker
- Details panel showing IP addresses and other information
- Port table showing open ports when network is selected

### Firewall Rules Widget
- Table view of firewall rules
- Filtering capabilities
- Rule details panel

### Routing Rules Widget
- Table view of routing table entries
- Visualization of network paths

### Docker Networks Widget
- List view of Docker networks
- Details panel for each network
- Container information

### Packet Sender Widget
- Source network selection dropdown
- Packet type selection (ping, HTTP)
- Destination input field
- Send button
- Response display area

## Security Considerations

1. **Elevated Privileges**:
   - Some operations require admin/root privileges
   - Implement proper error handling for permission issues
   - Provide clear messages to user about required privileges

2. **Input Validation**:
   - Validate all user inputs
   - Sanitize command arguments to prevent injection
   - Implement proper error handling

3. **Cross-Platform Compatibility**:
   - Handle platform-specific differences gracefully
   - Provide fallback mechanisms when certain features aren't available

## Error Handling

1. **System Command Execution**:
   - Handle command execution failures
   - Parse and display meaningful error messages
   - Implement timeouts for long-running operations

2. **Network Operations**:
   - Handle connection timeouts
   - Handle unreachable hosts
   - Implement retry mechanisms where appropriate

3. **UI Updates**:
   - Handle UI update failures gracefully
   - Provide feedback to user about operation status
   - Implement proper loading states

## Testing Strategy

1. **Unit Tests**:
   - Test data parsing functions
   - Test UI component logic
   - Mock system command execution

2. **Integration Tests**:
   - Test end-to-end functionality
   - Test cross-platform compatibility
   - Test privilege escalation scenarios

3. **UI Tests**:
   - Test UI component interactions
   - Test data binding
   - Test error display