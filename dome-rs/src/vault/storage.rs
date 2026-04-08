/// Vault storage and JSON serialization
/// 
/// Handles loading/saving vault and config files with JSON format

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

/// A single file/entry within a vault
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultFile {
    pub title: String,
    #[serde(rename = "type")]
    pub file_type: String, // "normal" or "password"
    pub content: String,
    pub encryption: String, // "normal"
}

/// The vault structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    pub files: Vec<VaultFile>,
    pub version: String,
}

/// A single vault configuration entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    pub name: String,
    pub path: String,
}

/// Application configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub vaults: Vec<VaultConfig>,
    pub version: String,
    pub updated_time: String,
}

impl Vault {
    /// Create a new empty vault
    pub fn new() -> Self {
        Self {
            files: vec![],
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Load vault from JSON bytes
    pub fn from_json(json: &str) -> Result<Self> {
        let vault = serde_json::from_str(json)?;
        Ok(vault)
    }

    /// Serialize vault to JSON
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Upgrade vault from older versions
    pub fn upgrade(&mut self) -> Result<()> {
        // TODO: Handle version migration if needed
        self.version = env!("CARGO_PKG_VERSION").to_string();
        Ok(())
    }
}

impl Config {
    /// Create new empty configuration
    pub fn new() -> Self {
        Self {
            vaults: vec![],
            version: env!("CARGO_PKG_VERSION").to_string(),
            updated_time: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Load config from JSON bytes
    pub fn from_json(json: &str) -> Result<Self> {
        let config = serde_json::from_str(json)?;
        Ok(config)
    }

    /// Serialize config to JSON
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_json_roundtrip() -> Result<()> {
        let mut vault = Vault::new();
        vault.files.push(VaultFile {
            title: "Gmail".to_string(),
            file_type: "password".to_string(),
            content: "my_secure_password".to_string(),
            encryption: "normal".to_string(),
        });

        let json = vault.to_json()?;
        let loaded = Vault::from_json(&json)?;

        assert_eq!(vault.files[0].title, loaded.files[0].title);
        Ok(())
    }
}
