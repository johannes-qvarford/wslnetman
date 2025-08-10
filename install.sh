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