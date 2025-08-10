# WSLNetMan Project Structure

## Directory Layout

```
wslnetman/
├── Cargo.toml
├── README.md
├── LICENSE
├── install.sh
├── .github/
│   └── workflows/
│       └── ci.yml
├── src/
│   ├── main.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   └── main_ui.slint
│   ├── network/
│   │   ├── mod.rs
│   │   ├── windows/
│   │   │   ├── mod.rs
│   │   │   ├── network_discovery.rs
│   │   │   ├── firewall_rules.rs
│   │   │   ├── routing_rules.rs
│   │   │   └── docker_networks.rs
│   │   └── wsl/
│   │       ├── mod.rs
│   │       ├── network_discovery.rs
│   │       ├── firewall_rules.rs
│   │       └── routing_rules.rs
│   └── packet/
│       ├── mod.rs
│       ├── ping.rs
│       ├── http_tcp.rs
│       └── sender.rs
└── tests/
    └── integration_tests.rs
```

## Key Files

### Cargo.toml
- Project metadata
- Slint dependency
- Network inspection dependencies
- Packet sending dependencies
- Testing dependencies

### src/main.rs
- Main application entry point
- Initializes Slint UI
- Sets up data models
- Connects UI events to backend functions

### src/ui/main_ui.slint
- Main UI layout
- Network widget
- Firewall rules widget
- Routing rules widget
- Docker networks widget
- Packet sender widget

### src/network/
- Modules for network information gathering
- Separate implementations for Windows and WSL
- Common interfaces for data structures

### src/packet/
- Ping implementation
- HTTP over TCP implementation
- Network interface selection
- Response handling

## Module Organization

### UI Module
- Contains all Slint UI definitions
- Handles UI state management
- Connects UI events to backend functions

### Network Module
- Platform-specific implementations
- Common data structures for network information
- Error handling for system command execution

### Packet Module
- Implements packet sending functionality
- Handles network interface binding
- Manages response collection and display