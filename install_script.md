# Install Script Documentation

## Purpose

The install.sh script sets up a pre-commit hook that automatically checks code formatting, runs Clippy linting, and executes tests before each commit. This ensures code quality and prevents broken code from being committed.

## Script Location

`install.sh`

## Script Content

```bash
#!/bin/bash

# WSLNetMan Install Script
# Sets up a pre-commit hook for code quality checks

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'  # No Color

echo -e "${GREEN}WSLNetMan Install Script${NC}"
echo "Setting up pre-commit hook..."

# Check if we're in the git repository
if [ ! -d ".git" ]; then
    echo -e "${RED}Error: This script must be run from the root of the git repository${NC}"
    exit 1
fi

# Create the pre-commit hook directory if it doesn't exist
mkdir -p .git/hooks

# Create the pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash

# Pre-commit hook for WSLNetMan
# Runs formatting check, clippy, and tests

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'  # No Color

echo -e "${YELLOW}Running pre-commit checks...${NC}"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed${NC}"
    exit 1
fi

# Check code formatting
echo -e "${YELLOW}Checking code formatting...${NC}"
if ! cargo fmt -- --check; then
    echo -e "${RED}Error: Code formatting check failed${NC}"
    echo -e "${YELLOW}Run 'cargo fmt' to fix formatting issues${NC}"
    exit 1
fi

# Run Clippy linting
echo -e "${YELLOW}Running Clippy linting...${NC}"
if ! cargo clippy -- -D warnings; then
    echo -e "${RED}Error: Clippy check failed${NC}"
    exit 1
fi

# Run tests
echo -e "${YELLOW}Running tests...${NC}"
if ! cargo test; then
    echo -e "${RED}Error: Tests failed${NC}"
    exit 1
fi

echo -e "${GREEN}All pre-commit checks passed!${NC}"
EOF

# Make the pre-commit hook executable
chmod +x .git/hooks/pre-commit

echo -e "${GREEN}Pre-commit hook installed successfully!${NC}"
echo -e "${YELLOW}The hook will automatically run:${NC}"
echo -e "${YELLOW}  - cargo fmt -- --check${NC}"
echo -e "${YELLOW}  - cargo clippy -- -D warnings${NC}"
echo -e "${YELLOW}  - cargo test${NC}"
echo ""
echo -e "${GREEN}Happy coding!${NC}"
```

## How It Works

1. **Script Execution**:
   - The script must be run from the root of the git repository
   - It checks for the existence of the `.git` directory

2. **Hook Installation**:
   - Creates the `.git/hooks` directory if it doesn't exist
   - Creates a `pre-commit` hook file with the checking logic
   - Makes the hook executable with `chmod +x`

3. **Pre-commit Checks**:
   - **Formatting Check**: Runs `cargo fmt -- --check` to ensure code is properly formatted
   - **Clippy Linting**: Runs `cargo clippy -- -D warnings` to catch potential issues
   - **Tests**: Runs `cargo test` to ensure all tests pass

4. **Error Handling**:
   - Each check exits with an error code if it fails
   - Provides helpful error messages and suggestions
   - Uses colored output for better visibility

## Usage

1. Make the script executable:
   ```bash
   chmod +x install.sh
   ```

2. Run the script:
   ```bash
   ./install.sh
   ```

3. The pre-commit hook will automatically run on every commit

## Customization

The script can be customized by modifying the pre-commit hook content:
- Add additional checks (e.g., documentation generation)
- Modify the checks that are run
- Change the output formatting

## Troubleshooting

### Common Issues

1. **"This script must be run from the root of the git repository"**:
   - Make sure you're in the correct directory
   - Verify the presence of the `.git` directory

2. **"cargo is not installed"**:
   - Install Rust using rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

3. **Formatting errors**:
   - Run `cargo fmt` to automatically fix formatting issues

4. **Clippy warnings**:
   - Address each warning individually
   - Use `#[allow(clippy::warning_name)]` for false positives

### Skipping the Hook

If you need to skip the pre-commit hook for a specific commit:
```bash
git commit --no-verify -m "Your commit message"
```

Note that this should be used sparingly and only when necessary.