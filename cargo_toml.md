# Cargo.toml Documentation

## Project Metadata

This document describes the contents of the Cargo.toml file for the WSLNetMan project.

### File Content

```toml
[package]
name = "wslnetman"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A GUI tool for inspecting and troubleshooting network issues between WSL and Windows"
license = "MIT"
repository = "https://github.com/your-username/wslnetman"
keywords = ["wsl", "network", "gui", "troubleshooting", "windows"]
categories = ["gui", "network-programming"]

[dependencies]
# Slint for GUI
slint = "1.0"

# Async runtime
tokio = { version = "1", features = ["full"] }

# HTTP client for packet sending
reqwest = "0.11"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# TCP utilities
tokio-util = { version = "0.7", features = ["codec"] }

# Futures
futures = "0.3"

# Command line parsing (if needed)
clap = { version = "4.0", features = ["derive"] }

# Cross-platform utilities
[target.'cfg(windows)'.dependencies]
# Windows-specific dependencies
windows = { version = "0.52", features = [
    "Win32_NetworkManagement_IpHelper",
    "Win32_Foundation",
    "Win32_System_Threading",
    "Win32_Networking_WinSock",
] }

[target.'cfg(unix)'.dependencies]
# Unix-specific dependencies
nix = "0.27"
socket2 = "0.5"

[build-dependencies]
slint-build = "1.0"

[dev-dependencies]
# Testing dependencies
tempfile = "3.0"
```

### Dependency Explanation

1. **slint**: The GUI framework for creating the user interface
2. **tokio**: Async runtime for handling concurrent operations
3. **reqwest**: HTTP client for sending HTTP packets
4. **serde/serde_json**: Serialization library for handling JSON data
5. **tokio-util**: Utilities for working with TCP streams
6. **futures**: Future combinators and utilities
7. **clap**: Command line argument parser (if CLI features are added)
8. **windows**: Windows API bindings for Windows-specific functionality
9. **nix**: Unix API bindings for Unix-specific functionality
10. **socket2**: Low-level networking library for socket operations
11. **slint-build**: Build-time dependencies for Slint
12. **tempfile**: Temporary file creation for testing

### Features

The Cargo.toml file uses conditional compilation features:
- Windows-specific dependencies are only compiled on Windows
- Unix-specific dependencies are only compiled on Unix-like systems (including WSL)

### Build Configuration

The project uses the 2021 edition of Rust and includes build dependencies for Slint integration.