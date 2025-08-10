# CI Workflow Documentation

## GitHub Actions Workflow

This document describes the GitHub Actions workflow that will be used for continuous integration.

### Workflow File Location

`.github/workflows/ci.yml`

### Workflow Configuration

```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable
        override: true
        
    - name: Install system dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y libgtk-3-dev
        
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Cache cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Cache build artifacts
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Build
      run: cargo build --verbose
      
    - name: Run tests
      run: cargo test --verbose
      
    - name: Check formatting
      run: cargo fmt -- --check
      
    - name: Run Clippy
      run: cargo clippy -- -D warnings
```

### Workflow Details

1. **Triggers**:
   - Push to main branch
   - Pull requests to main branch

2. **Jobs**:
   - Build job running on Ubuntu latest

3. **Steps**:
   - Checkout code
   - Install Rust toolchain
   - Install system dependencies (GTK for Slint)
   - Cache cargo registry, index, and build artifacts
   - Build the project
   - Run tests
   - Check code formatting
   - Run Clippy linter

### Environment Variables

- `CARGO_TERM_COLOR`: Always use colored output for cargo commands

### Caching Strategy

The workflow uses GitHub Actions caching for:
- Cargo registry (`~/.cargo/registry`)
- Cargo git index (`~/.cargo/git`)
- Build artifacts (`target` directory)

This significantly speeds up subsequent builds by reusing previously downloaded dependencies and compiled artifacts.

### Security Considerations

- The workflow runs on GitHub-hosted runners
- No sensitive information is stored in the workflow
- All dependencies are fetched from official sources