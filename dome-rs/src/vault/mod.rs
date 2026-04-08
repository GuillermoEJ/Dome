/// Vault data structures and management
/// 
/// Handles loading, saving, and upgrading vault files

pub mod storage;
pub mod manager;

pub use storage::{Vault, VaultFile, Config};
pub use manager::VaultManager;

use serde::{Deserialize, Serialize};

/// Vault file entry types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Normal,
    Password,
}

/// Encryption type (for future extensibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EncryptionType {
    Normal,
}
