# WSLNetMan (WSL Network Manager)

[![CI](https://github.com/your-username/wslnetman/actions/workflows/ci.yml/badge.svg)](https://github.com/your-username/wslnetman/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

WSLNetMan is a graphical tool for inspecting and troubleshooting network issues between Windows and WSL (Windows Subsystem for Linux). It provides a unified interface to view network configurations, firewall rules, routing tables, and Docker networks across both environments.

## Features

### Network Inspection
- View all network interfaces on Windows, WSL, and Docker containers
- Display IP addresses and network labels for easy identification
- Show open ports with process information (Process ID, Process Name, Protocol, Direction)

### Firewall Rules
- Inspect Windows firewall rules that may affect WSL networking
- View rule details including direction, action, and address filtering

### Routing Tables
- Examine routing rules on both Windows and WSL
- Understand how network traffic is routed between environments

### Docker Networks
- View Docker networks running within WSL
- Inspect container network configurations

### Packet Sender
- Send ping requests from specific network interfaces
- Send HTTP requests over TCP from selected networks
- View responses from sent packets

## Screenshots

_TODO: Add screenshots of the application UI_

## Installation

### Prerequisites

- Windows 10/11 with WSL2
- Rust toolchain
- Slint GUI framework dependencies
- Docker (for Docker network inspection features)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/your-username/wslnetman.git
cd wslnetman

# Build the project
cargo build --release

# Run the application
cargo run --release
```

### Installation Script

Run the installation script to set up a pre-commit hook that checks formatting, clippy, and runs tests:

```bash
./install.sh
```

## Usage

1. Launch the application
2. Use the tabbed interface to navigate between different inspection tools
3. Click "Refresh" to update all network information
4. Select networks to view detailed port information
5. Use the Packet Sender tab to send test packets from specific networks

## Development

### Project Structure

```
wslnetman/
├── Cargo.toml          # Project dependencies and metadata
├── src/                # Source code
│   ├── main.rs         # Application entry point
│   ├── ui/             # Slint UI components
│   ├── network/        # Network inspection modules
│   └── packet/         # Packet sending functionality
├── tests/              # Integration tests
└── .github/workflows/  # CI configuration
```

### Building the UI

The UI is built using [Slint](https://slint.rs/), a toolkit for building native GUI applications. The UI components are defined in `.slint` files in the `src/ui/` directory.

### Running Tests

```bash
# Run unit tests
cargo test

# Run clippy for linting
cargo clippy

# Check code formatting
cargo fmt -- --check
```

### GitHub Actions

The project includes GitHub Actions workflows that automatically run:
- Code formatting checks
- Clippy linting
- Unit tests

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Slint](https://slint.rs/) - The GUI framework used in this project
- [Rust](https://www.rust-lang.org/) - The programming language
- Microsoft for WSL (Windows Subsystem for Linux)

## Troubleshooting

### Common Issues

1. **Permission Errors**: Some network inspection features require elevated privileges. Run the application as administrator on Windows or with sudo on WSL.

2. **Docker Connection Issues**: Ensure Docker is running and you have permission to access the Docker daemon.

3. **WSL Network Not Detected**: Make sure WSL is properly installed and at least one distribution is installed.

### Getting Help

If you encounter any issues, please [open an issue](https://github.com/your-username/wslnetman/issues) on GitHub.


## Packaging (MSI) and CI artifact lookup

If your CI or local scripts need to locate the generated MSI, prefer using the provided PowerShell helper to avoid null-path errors when a specific target triple directory does not exist:

```powershell
# Returns the full path to the newest MSI under ./target
$MSI_PATH = powershell -NoProfile -ExecutionPolicy Bypass -File .\find-msi.ps1

# Safely derive the file name
if (-not [string]::IsNullOrWhiteSpace($MSI_PATH)) {
  $MSI_NAME = Split-Path $MSI_PATH -Leaf
  Write-Host "MSI: $MSI_NAME ($MSI_PATH)"
} else {
  Write-Error "No MSI was found. Ensure your packaging step produced an .msi under 'target'."
  exit 1
}
```

The helper searches these locations (in order):
- target\x86_64-pc-windows-msvc\wix\*.msi
- target\x86_64-pc-windows-gnu\wix\*.msi
- target\**\*.msi (fallback)

This prevents errors like "Cannot bind argument to parameter 'Path' because it is null" when Split-Path is used on a missing MSI path.
