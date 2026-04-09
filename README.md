# DOME - Data Obfuscation & Management Engine

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Production%20Ready-brightgreen)]()
[![Security](https://img.shields.io/badge/Security-AES--256-critical)]()

A lightweight, secure, and fast file manager written in Rust.

> 🔐 **High-level security** | ⚡ **Native performance** | 📦 **Single binary** | 🛡️ **Memory-safe by design**

## Features

### 🔐 Security
- **AES-256 CFB** encryption with random IV
- **PBKDF2-SHA256** key derivation
- **Zlib compression** for storage efficiency
- No plaintext passwords stored or logged
- Memory-safe Rust implementation

### 💾 Core Functionality
- Create, edit, and delete password entries
- Full-text search by title and content
- Auto-generate secure passwords (16+ characters)
- Import/export support
- Multiple vault support
- Automatic encryption on save

### ⚙️ Architecture
- **Modular design** - Separate crypto, vault, CLI, and storage layers
- **Cross-platform** - Works on Windows, macOS, Linux, and ARM
- **Async-ready** - Built with tokio for future enhancements
- **Type-safe** - Leverages Rust's type system for correctness

## Installation

### macOS / Linux

```bash
# Build from source
git clone https://github.com/GuillermoEJ/dome.git
cd dome/dome-rs
cargo build --release
./target/release/dome
```

### Windows

**Option 1: Automated script**
```powershell
powershell -ExecutionPolicy Bypass -File dome-rs/build.ps1
```

**Option 2: Manual build**
```powershell
git clone https://github.com/GuillermoEJ/dome.git
cd dome\dome-rs
cargo build --release
.\target\release\dome.exe
```

### From Source

```bash
# Prerequisites: Rust 1.75+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
git clone https://github.com/GuillermoEJ/dome.git
cd dome/dome-rs
cargo build --release

# Binary location
./target/release/dome  # macOS/Linux
.\target\release\dome.exe  # Windows
```

## Commands

| Command | Usage | Description |
|---------|-------|-------------|
| `new` | `new <title>` | Create new entry |
| `edit` | `edit <title>` | Edit entry content |
| `remove` | `remove <title>` | Delete entry |
| `list`, `ls` | — | List all entries |
| `search` | `search <term>` | Search by title |
| `searchcontents` | `searchcontents <term>` | Search content |
| `append` | `append <title> <text>` | Append to entry |
| `rename` | `rename <old> <new>` | Rename entry |
| `passcreate` | — | Generate random password |
| `passrefresh` | `passrefresh <title>` | Generate new password |
| `printeverything` | — | Display all (unencrypted!) |
| `dbencrypt` | — | Re-encrypt vault |
| `clear`, `cls` | — | Clear screen |
| `help` | — | Show help |
| `exit`, `quit` | — | Save and exit |

## Building

### Development (Fast Compile)
```bash
cd dome-rs
cargo build

# Run debug binary
./target/debug/dome
```

## Testing

```bash
cd dome-rs

# Run all tests
cargo test

# Run crypto tests (security critical)
cargo test crypto --lib

# Run with output
cargo test -- --nocapture

# Test coverage
cargo tarpaulin
```

## Security

### Threat Model
- **Protects against**: Data breaches, disk theft, unauthorized access
- **Assumes**: Secure password, no keyloggers, local machine
- **Uses**: AES-256 (NIST approved, NSA Suite B)
- **Derives keys**: PBKDF2-SHA256 (100,000 iterations)

### File Permissions
- Unix/Linux: Vault files created with `0600` (user read/write only)
- Windows: Uses NTFS permissions (inherits parent directory)

### Password Security
- Passwords never logged or printed
- Input masked (hidden) during entry
- Keys zero'd from memory after use (future enhancement with `zeroize`)
- No temporary files with plaintext

## Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| **Linux** | Tested | x86_64, ARM64 |
| **macOS** | Tested | Intel, Apple Silicon (M1/M2) |
| **Windows** | Tested | Windows 10+ |
| **BSD** | Compatible | Not officially tested |

## Dependencies

### Production
- `aes` - AES encryption
- `pbkdf2` - Key derivation
- `serde` & `serde_json` - Serialization
- `rustyline` - Interactive REPL
- `colored` - Terminal colors
- `flate2` - Zlib compression

See [dome-rs/Cargo.toml](dome-rs/Cargo.toml) for complete dependency list (23 crates total).

### Zero External Crypto Dependencies
All cryptographic primitives use only:
- `aes` (standard library-backed implementation)
- `pbkdf2` (standard NIST algorithm)
- `sha2` (standard secure hash)

All dependencies are well-maintained and widely-used in production.

## Roadmap

- [ ] Multi-vault switching
- [ ] CSV import/export
- [ ] Backup encryption
- [ ] Master password change
- [ ] Web UI (optional desktop view)
- [ ] Cloud sync (encrypted, E2E)
- [ ] API for integration
- [ ] 2-factor authentication

## Contributing

Contributions welcome! Areas of interest:
- Security audits and hardening
- Performance optimization
- UI/UX improvements
- Platform-specific enhancements
- Documentation

### Development Setup
```bash
# Clone and setup
git clone https://github.com/GuillermoEJ/dome.git
cd dome/dome-rs

# Verify setup
cargo build
cargo test

# Format and lint
cargo fmt
cargo clippy
```

## License

MIT License - See [LICENSE](LICENSE) file for details.


## Support

- **Issues**: [GitHub Issues](https://github.com/GuillermoEJ/dome/issues)
- **Discussions**: [GitHub Discussions](https://github.com/GuillermoEJ/dome/discussions)


