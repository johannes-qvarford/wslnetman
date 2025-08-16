# WSL Network Manager - Claude Context

## Project Overview
WSL Network Manager is a GUI application for inspecting and troubleshooting network issues between WSL and Windows environments. It provides network interface discovery, Docker network inspection, port monitoring, packet sending capabilities, clipboard integration, and process management.

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

### Testing and Development
```bash
# Build and run Windows version (recommended for testing)
./build-and-run-windows.sh

# Or build and run separately
./build-windows.sh
./run-windows.sh
```

### PowerShell Scripts (Windows)
For Windows users, PowerShell equivalents are available:

```powershell
# Install pre-commit hooks for code quality
.\install.ps1

# Fix code formatting and apply Clippy suggestions
.\fix-app.ps1
```

The install script sets up Git pre-commit hooks that automatically run:
- `cargo fmt -- --check`: Code formatting verification
- `cargo clippy -- -D warnings`: Linting with warnings treated as errors
- `cargo test`: Full test suite

### Cross-Compilation for Windows
The project includes scripts for cross-compiling to Windows and running the native Windows version from WSL:

```bash
# Build Windows executable (requires mingw-w64)
./build-windows.sh [debug|release]

# Build and run Windows version directly
./run-windows.sh [debug|release]

# Or run pre-built Windows executable
./target/x86_64-pc-windows-gnu/release/wslnetman.exe
```

#### Prerequisites for Windows Cross-Compilation
- MinGW-w64 toolchain: `sudo apt install mingw-w64`
- Windows target: `rustup target add x86_64-pc-windows-gnu` (automatically installed by build script)

#### Windows vs WSL Runtime Behavior
- **WSL Version**: Uses `powershell.exe` calls for Windows network discovery (WSL interop)
- **Windows Version**: Runs natively on Windows, direct PowerShell access
- **GUI**: Both versions use the same Slint-based interface
- **Network Discovery**: Windows version has more direct access to Windows networking APIs

## Current Implementation Status
- ✅ Network interface discovery (Windows + WSL + Docker)
- ✅ Active port discovery with filtering and search
- ✅ Docker network inspection with container details
- ✅ Packet sending (ping + HTTP)
- ✅ Advanced GUI with tabs, modals, and interactive elements
- ✅ Clipboard integration for copying network data
- ✅ Process management with safe process termination
- ✅ Port filtering by process name, port number, or PID
- ✅ Windows icon embedding and native Windows support

## UI Features
- **Modal System**: Detailed views for network interfaces and Docker networks
- **Interactive Tables**: Clickable elements for copying data to clipboard
- **Search and Filter**: Real-time filtering of ports and processes
- **Process Management**: Kill processes with safety restrictions (excludes system processes)
- **Responsive Design**: Scrollable content areas for large datasets
- **Color Coding**: Visual indicators for network states, environments, and status

## Dependencies
### Core Dependencies
- `slint`: GUI framework (v1.0)
- `tokio`: Async runtime with full features
- `reqwest`: HTTP client for packet sending
- `serde` + `serde_json`: JSON serialization for command outputs
- `arboard`: Cross-platform clipboard access
- `clap`: Command line argument parsing
- `tokio-util`: Additional Tokio utilities with codec features
- `futures`: Future combinators and utilities

### Platform-Specific Dependencies
- **Windows**: `windows` crate for networking APIs and system integration
- **Build**: `winresource` for embedding Windows icons

### Development Dependencies
- `slint-build`: Slint UI compilation
- `tempfile`: Testing utilities

## WSL2 Port Forwarding Context
The application runs in WSL2 and can demonstrate WSL2's automatic port forwarding mechanism:
- Docker containers binding to `0.0.0.0:port` in WSL become accessible from Windows `localhost:port`
- WSL spawns `wslrelay.exe` processes on Windows to handle forwarding
- Windows processes take priority over WSL for port binding conflicts

## Development Notes
- Use system commands instead of low-level networking where possible for simplicity
- Mock/default data is provided when actual system commands fail
- UI updates via callbacks from async operations
- Cross-platform compatibility handled at compile time with `#[cfg]` attributes
- Clipboard operations are handled gracefully with error handling for unsupported platforms
- Process termination includes safety restrictions to prevent killing critical system processes
- Modal system prevents accidental closure and provides clear navigation
- Build script automatically embeds Windows icons for native Windows distributions

## Security Considerations
- **Process Termination**: Safety restrictions prevent killing system-critical processes (PIDs 0, 1, 4, and N/A)
- **Command Execution**: All system commands are executed with proper sanitization
- **Cross-Platform Safety**: Windows-specific code is conditionally compiled and isolated