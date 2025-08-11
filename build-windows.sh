#!/bin/bash
set -e

echo "🔨 WSL Network Manager - Windows Cross-Compilation Script"
echo "=========================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Target architecture
TARGET="x86_64-pc-windows-gnu"

# Check if target is installed
echo -e "${BLUE}Checking Rust target: ${TARGET}${NC}"
if ! rustup target list --installed | grep -q "${TARGET}"; then
    echo -e "${YELLOW}Installing Windows target: ${TARGET}${NC}"
    rustup target add "${TARGET}"
else
    echo -e "${GREEN}✓ Target ${TARGET} already installed${NC}"
fi

# Check for MinGW toolchain
echo -e "${BLUE}Checking MinGW toolchain...${NC}"
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo -e "${RED}❌ MinGW toolchain not found!${NC}"
    echo "Please install mingw-w64:"
    echo "  sudo apt install mingw-w64"
    exit 1
fi
echo -e "${GREEN}✓ MinGW toolchain found${NC}"

# Set up environment for cross-compilation
export PKG_CONFIG_ALLOW_CROSS=1
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++

# Build configuration
BUILD_TYPE="${1:-release}"

if [ "$BUILD_TYPE" = "debug" ]; then
    echo -e "${BLUE}Building debug version for Windows...${NC}"
    CARGO_CMD="cargo build --target ${TARGET}"
    BINARY_PATH="target/${TARGET}/debug/wslnetman.exe"
elif [ "$BUILD_TYPE" = "release" ]; then
    echo -e "${BLUE}Building release version for Windows...${NC}"
    CARGO_CMD="cargo build --target ${TARGET} --release"
    BINARY_PATH="target/${TARGET}/release/wslnetman.exe"
else
    echo -e "${RED}❌ Invalid build type: ${BUILD_TYPE}${NC}"
    echo "Usage: $0 [debug|release]"
    exit 1
fi

# Show build environment info
echo -e "${BLUE}Build Environment:${NC}"
echo "  Target: ${TARGET}"
echo "  Build Type: ${BUILD_TYPE}"
echo "  Linker: x86_64-w64-mingw32-gcc"
echo "  Output: ${BINARY_PATH}"
echo ""

# Build the project
echo -e "${BLUE}Starting build...${NC}"
if ${CARGO_CMD}; then
    echo ""
    echo -e "${GREEN}✅ Build completed successfully!${NC}"
    echo -e "${GREEN}Windows executable: ${BINARY_PATH}${NC}"
    
    # Check if binary exists and show size
    if [ -f "${BINARY_PATH}" ]; then
        BINARY_SIZE=$(du -h "${BINARY_PATH}" | cut -f1)
        echo -e "${GREEN}Binary size: ${BINARY_SIZE}${NC}"
    fi
    
    echo ""
    echo -e "${BLUE}To run the Windows version:${NC}"
    echo "  ./${BINARY_PATH}"
    echo "  or use: ./run-windows.sh"
    
else
    echo -e "${RED}❌ Build failed!${NC}"
    exit 1
fi