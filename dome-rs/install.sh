#!/bin/bash
# Installation script for DOME - Secure Password Vault

set -e

echo "🚀 DOME Installation Script"
echo "============================"
echo ""

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Verify Rust installation
echo "✓ Rust version:"
rustc --version
cargo --version

# Navigate to project
cd "$(dirname "$0")/dome-rs"

echo ""
echo "📦 Building DOME..."
cargo build --release

echo ""
echo "✓ Build complete!"
echo ""
echo "📍 Compiled binary location:"
echo "   $(pwd)/target/release/dome"
echo ""
echo "🎯 Next steps:"
echo "   1. Move binary to PATH (optional):"
echo "      sudo cp target/release/dome /usr/local/bin/"
echo ""
echo "   2. Run DOME:"
echo "      ./target/release/dome"
echo ""
echo "✅ Installation successful!"
