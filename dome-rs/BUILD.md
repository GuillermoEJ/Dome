# Building DOME

This document explains how to build DOME on different platforms.

## Prerequisites

- **Rust 1.70+** - Install from [https://rustup.rs/](https://rustup.rs/)
- **Build tools** - Platform-specific (see below)

## Windows (PowerShell)

### Automatic Build (Recommended)

The `build.ps1` script handles everything automatically, including:
- Installing MinGW-w64 if needed
- Setting up the Rust GNU toolchain
- Building the release binary

Run from the `dome-rs` directory:

```powershell
powershell -ExecutionPolicy Bypass -File build.ps1
```

### Manual Build Steps

If you prefer to build manually:

1. **Install MinGW-w64** (if not already installed):
   ```powershell
   # Download: https://github.com/niXman/mingw-builds-binaries/releases/
   # Extract to C:\MinGW-w64
   # Add C:\MinGW-w64\mingw64\bin to your PATH
   ```

2. **Configure Rust**:
   ```powershell
   rustup target install x86_64-pc-windows-gnu
   rustup default stable-x86_64-pc-windows-gnu
   ```

3. **Build**:
   ```powershell
   cargo build --release
   ```

**Output**: `target/release/dome.exe`

---

## Linux

### Automated Installation

Run the installation script:

```bash
chmod +x install.sh
./install.sh
```

This will:
- Verify build tools are installed
- Install Rust (if needed)
- Build DOME in release mode
- Show installation options

### Manual Build

1. **Install dependencies** (choose your distro):

   **Ubuntu/Debian**:
   ```bash
   sudo apt-get update
   sudo apt-get install build-essential cargo rustc
   ```

   **Fedora/RHEL**:
   ```bash
   sudo dnf install gcc cargo rustc
   ```

   **Arch Linux**:
   ```bash
   sudo pacman -S base-devel cargo
   ```

2. **Clone and build**:
   ```bash
   cargo build --release
   ```

3. **Install** (optional):
   ```bash
   sudo cp target/release/dome /usr/local/bin/
   ```

**Output**: `target/release/dome`

---

## macOS

### Automated Installation

```bash
chmod +x install.sh
./install.sh
```

### Manual Build

1. **Install Xcode Command Line Tools**:
   ```bash
   xcode-select --install
   ```

2. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Build**:
   ```bash
   cargo build --release
   ```

4. **Install** (optional):
   ```bash
   cp target/release/dome /usr/local/bin/
   ```

**Output**: `target/release/dome`

---

## Using Make (Linux/macOS)

If you have `make` installed, build commands are simplified:

```bash
make release          # Build optimized release
make run              # Build and run debug version
make test             # Run test suite
make install          # Install to ~/.local/bin
make clean            # Remove build artifacts
make help             # See all commands
```

---

## Build Variants

### Debug Build (Faster, Larger)
```bash
cargo build
# Output: target/debug/dome
```

### Release Build (Slower, Optimized, Smaller)
```bash
cargo build --release
# Output: target/release/dome (or dome.exe on Windows)
```

### Check Without Building
```bash
cargo check
# Useful for quickly detecting compilation errors
```

---

## Testing

Run the full test suite:

```bash
cargo test
```

Run only crypto tests:

```bash
cargo test crypto
```

---

## Troubleshooting

### Windows: `dlltool not found`
- Install MinGW-w64: https://github.com/niXman/mingw-builds-binaries/
- Or re-run `build.ps1` which installs it automatically

### Linux: `command not found: cargo`
- Either:
  1. Run `rustup` installer from https://rustup.rs/
  2. Or use your package manager: `apt-get install cargo` (Debian/Ubuntu)

### All Platforms: Compilation Errors
- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build --release`
- Check Rust version: `rustc --version` (should be 1.70+)

---

## Build Configuration

Key dependencies in `Cargo.toml`:
- **Cryptography**: `aes`, `cfb-mode`, `sha2`, `pbkdf2`
- **Compression**: `flate2`
- **UI**: `ratatui`, `crossterm`
- **Utilities**: `serde`, `tokio`, `anyhow`

---

## Build Output

After successful compilation:

- **Windows**: `.exe` file at `target/release/dome.exe`
- **Linux/macOS**: Binary at `target/release/dome`

To verify the build:
```bash
./target/release/dome --version
```

Or run directly:
```bash
./target/release/dome
```

---

## Cross-Platform Notes

- DOME uses Rust's stable toolchain for maximum compatibility
- Windows requires GNU toolchain (not MSVC) due to cfb-mode cryptography crate
- Linux/macOS work with any standard C compiler (gcc, clang)

For detailed build output, add `-v` flag:
```bash
cargo build --release -v
```
