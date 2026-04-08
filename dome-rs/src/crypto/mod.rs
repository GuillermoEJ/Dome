/// Cryptographic functions for DOME
/// Handles AES-256 encryption/decryption with PBKDF2 key derivation

pub mod cipher;
pub mod kdf;

pub use cipher::{encrypt, decrypt};
pub use kdf::derive_key;

// Standard salt value (same as Python version for compatibility)
pub const SALT: &[u8] = b"\x8a\xfe\x1f\xa7aY}\xa3It=\xc3\xccT\xc8\x94\xc11%w]A\xb7\x87G\xd8\xba\x9e\xf8\xec&\xf0";
pub const KEY_LENGTH: usize = 32; // 256 bits for AES-256
