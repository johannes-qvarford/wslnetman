# WSL Network Manager - Claude Context

## Project Overview
WSL Network Manager is a GUI application for inspecting and troubleshooting network issues between WSL and Windows environments.
It provides network interface discovery, Docker network inspection, port monitoring, packet sending capabilities,
clipboard integration, and process management.

It's only meant to be run on Windows.

## Architecture
- **GUI Framework**: Slint (Rust GUI toolkit)
- **Runtime**: Tokio async runtime
- **Network**: Uses system commands (`ip addr`, `docker network ls`, `ss`, `powershell.exe`) 
- **Cross-platform**: Runs in WSL2 but can inspect both WSL and Windows networks
- **Process Management**: Can kill processes by PID (with safety restrictions)
- **Clipboard Integration**: Copy network information to system clipboard

## Core Functionality
1. **Network Tab**: Displays network interfaces from Windows, WSL, and Docker with detailed modal views
2. **Docker Tab**: Shows Docker networks and their configurations with container details
3. **Packet Sender Tab**: Send ping/HTTP packets for network testing
4. **Port Filtering**: Search and filter ports by process, port number, or PID
5. **Process Management**: Kill processes associated with specific ports (with safety restrictions)
6. **Clipboard Operations**: Copy IP addresses, MAC addresses, port numbers, and other network data

## Key Files
- `src/main.rs`: Main application entry point with UI callbacks
- `src/ui/main_ui.slint`: Main Slint UI definition
- `src/ui/components/`: UI component modules
  - `network_tab.slint`: Network interface display tab
  - `docker_tab.slint`: Docker network display tab
  - `network_detail_modal.slint`: Detailed network interface modal with port filtering
  - `docker_network_detail_modal.slint`: Detailed Docker network modal with container info
- `src/ui/styles/table.slint`: Reusable table styling
- `src/ui/types.slint`: Shared type definitions
- `src/network/mod.rs`: Core network discovery functions
- `src/network/wsl/mod.rs`: WSL-specific network discovery
- `src/network/windows/mod.rs`: Windows network discovery via PowerShell
- `src/network/docker.rs`: Docker network discovery
- `src/packet/`: Packet sending functionality (ping, HTTP)
  - `ping.rs`: Ping packet implementation
  - `http.rs`: HTTP request implementation
  - `response_display.md`: UI response display documentation
- `build.rs`: Build script with Windows icon embedding

## Build & Development

### Standard WSL/Linux Build
```bash
# Build the project
cargo build

# Run the application
cargo run

# Run tests
cargo test

# Check for issues
cargo check
cargo clippy
```

### PowerShell Scripts (Windows)

```powershell
# Install pre-commit hooks for code quality
.\install.ps1

# Fix code formatting and apply Clippy suggestions
.\fix-app.ps1
```

The install script sets up Git pre-commit hooks that automatically run:
- `cargo fmt -- --check`: Code formatting verification
- `cargo clippy -- -D warnings`: Linting with warnings treated as errors

## UI Features
- **Modal System**: Detailed views for network interfaces and Docker networks
- **Interactive Tables**: Clickable elements for copying data to clipboard
- **Search and Filter**: Real-time filtering of ports and processes
- **Process Management**: Kill processes with safety restrictions (excludes system processes)
- **Responsive Design**: Scrollable content areas for large datasets
- **Color Coding**: Visual indicators for network states, environments, and status