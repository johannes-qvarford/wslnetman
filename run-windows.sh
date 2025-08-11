#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 WSL Network Manager - Windows Runner${NC}"
echo "========================================"

# Target architecture
TARGET="x86_64-pc-windows-gnu"

# Check for build type argument
BUILD_TYPE="${1:-release}"
if [ "$BUILD_TYPE" != "debug" ] && [ "$BUILD_TYPE" != "release" ]; then
    # If first argument is not build type, treat it as direct exe path or assume release
    if [ -f "$1" ]; then
        BINARY_PATH="$1"
        shift # Remove the binary path from arguments
    else
        BUILD_TYPE="release"
    fi
fi

# Set binary path if not already set
if [ -z "$BINARY_PATH" ]; then
    if [ "$BUILD_TYPE" = "debug" ]; then
        BINARY_PATH="target/${TARGET}/debug/wslnetman.exe"
    else
        BINARY_PATH="target/${TARGET}/release/wslnetman.exe"
    fi
fi

# Check if binary exists, if not try to build it
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${YELLOW}Windows executable not found at: ${BINARY_PATH}${NC}"
    echo -e "${BLUE}Building Windows version...${NC}"
    
    if [ -f "./build-windows.sh" ]; then
        ./build-windows.sh "$BUILD_TYPE"
    else
        echo -e "${RED}❌ build-windows.sh script not found!${NC}"
        echo "Please run the build script first or ensure you're in the project root directory."
        exit 1
    fi
fi

# Verify binary exists after potential build
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}❌ Windows executable not found: ${BINARY_PATH}${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Found Windows executable: ${BINARY_PATH}${NC}"

# Display some info about the binary
BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
echo -e "${BLUE}Binary size: ${BINARY_SIZE}${NC}"

# WSL can run Windows executables directly
echo -e "${BLUE}Launching Windows application...${NC}"
echo -e "${YELLOW}Note: This will run as a native Windows application${NC}"
echo ""

# Execute the Windows binary with any remaining arguments
# WSL handles the Windows executable execution transparently
exec "$BINARY_PATH" "$@"