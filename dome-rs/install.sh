#!/bin/bash
# Installation script for DOME - Secure Password Vault

set -e

echo "DOME Installation Script"
echo "============================"
echo ""

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    echo "Detected: Linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
    echo "Detected: macOS"
else
    echo "Error: Unsupported OS. Use build.ps1 on Windows."
    exit 1
fi

# Check build tools
if ! command -v gcc &> /dev/null; then
    echo ""
    echo "Error: GCC not found. Install build essentials:"
    if [ "$OS" = "linux" ]; then
        echo "  Ubuntu/Debian: sudo apt-get install build-essential"
        echo "  Fedora/RHEL: sudo dnf install gcc"
    elif [ "$OS" = "macos" ]; then
        echo "  macOS: xcode-select --install"
    fi
    exit 1
fi

echo "Build tools verified"

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Verify Rust installation
echo ""
echo "Rust version:"
rustc --version
cargo --version

# Navigate to project
cd "$(dirname "$0")"

echo ""
echo "Building DOME (release mode)..."
cargo build --release

echo ""
echo "✅ Build complete!"
echo ""
echo "Binary location:"
echo "   $(pwd)/target/release/dome"
echo ""
echo "Installation options:"
echo "   1. Move to system PATH (requires sudo):"
echo "      sudo cp target/release/dome /usr/local/bin/"
echo ""
echo "   2. Add to ~/.local/bin (no sudo needed):"
echo "      mkdir -p ~/.local/bin"
echo "      cp target/release/dome ~/.local/bin/"
echo "      export PATH=~/.local/bin:\$PATH"
echo ""
echo "   3. Run directly:"
echo "      ./target/release/dome"
echo ""
echo "For more info: ./target/release/dome --help"
