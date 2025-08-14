# WSL Network Manager - Claude Context

## Project Overview
WSL Network Manager is a GUI application for inspecting and troubleshooting network issues between WSL and Windows environments. It provides network interface discovery, Docker network inspection, port monitoring, and packet sending capabilities.

## Architecture
- **GUI Framework**: Slint (Rust GUI toolkit)
- **Runtime**: Tokio async runtime
- **Network**: Uses system commands (`ip addr`, `docker network ls`, `ss`, `powershell.exe`) 
- **Cross-platform**: Runs in WSL2 but can inspect both WSL and Windows networks

## Core Functionality
1. **Network Tab**: Displays network interfaces from Windows, WSL, and Docker
2. **Docker Tab**: Shows Docker networks and their configurations
3. **Packet Sender Tab**: Send ping/HTTP packets for network testing

## Key Files
- `src/main.rs`: Main application entry point with UI callbacks
- `src/ui/main_ui.slint`: Slint UI definition
- `src/network/mod.rs`: Core network discovery functions
- `src/network/wsl/mod.rs`: WSL-specific network discovery
- `src/network/windows/mod.rs`: Windows network discovery via PowerShell
- `src/network/docker.rs`: Docker network discovery
- `src/packet/`: Packet sending functionality (ping, HTTP)

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
- ✅ Active port discovery
- ✅ Docker network inspection
- ✅ Packet sending (ping + HTTP)
- ✅ Basic GUI with tabs and refresh functionality

## Dependencies
- `slint`: GUI framework
- `tokio`: Async runtime
- `reqwest`: HTTP client for packet sending
- `serde`: JSON serialization for command outputs
- Platform-specific: `windows` crate for Windows, `nix` for Unix

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