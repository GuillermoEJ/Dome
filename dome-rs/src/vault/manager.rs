/// Vault management operations
/// 
/// High-level API for vault operations (list, add, remove, search, etc.)

use super::{Vault, VaultFile, Config};
use crate::crypto::{encrypt, decrypt};
use std::path::PathBuf;
use anyhow::{Result, anyhow};

pub struct VaultManager {
    vault: Vault,
    config: Config,
    vault_path: PathBuf,
    config_path: PathBuf,
}

impl VaultManager {
    /// Create a new vault manager
    pub fn new(vault_path: PathBuf, config_path: PathBuf) -> Self {
        Self {
            vault: Vault::new(),
            config: Config::new(),
            vault_path,
            config_path,
        }
    }

    /// List all files in vault
    pub fn list_files(&self) -> &[VaultFile] {
        &self.vault.files
    }

    /// Add a new file to vault
    pub fn add_file(&mut self, file: VaultFile) {
        self.vault.files.push(file);
    }

    /// Remove a file by title
    pub fn remove_file(&mut self, title: &str) -> Result<()> {
        if let Some(pos) = self.vault.files.iter().position(|f| f.title == title) {
            self.vault.files.remove(pos);
            Ok(())
        } else {
            Err(anyhow!("File not found: {}", title))
        }
    }

    /// Find files by search term
    pub fn search(&self, term: &str) -> Vec<&VaultFile> {
        self.vault
            .files
            .iter()
            .filter(|f| f.title.contains(term) || f.content.contains(term))
            .collect()
    }

    /// Save vault to disk (encrypted)
    pub fn save_vault(&self, password: &str) -> Result<()> {
        let json = self.vault.to_json()?;
        let encrypted = encrypt(json.as_bytes(), password)?;
        std::fs::write(&self.vault_path, encrypted)?;
        Ok(())
    }

    /// Load vault from disk (encrypted)
    pub fn load_vault(&mut self, password: &str) -> Result<()> {
        let encrypted = std::fs::read(&self.vault_path)?;
        let decrypted = decrypt(&encrypted, password)?;
        let json = String::from_utf8(decrypted)?;
        self.vault = Vault::from_json(&json)?;
        Ok(())
    }

    /// Save configuration to disk
    pub fn save_config(&self) -> Result<()> {
        let json = self.config.to_json()?;
        std::fs::write(&self.config_path, json)?;
        Ok(())
    }

    /// Load configuration from disk
    pub fn load_config(&mut self) -> Result<()> {
        if self.config_path.exists() {
            let json = std::fs::read_to_string(&self.config_path)?;
            self.config = Config::from_json(&json)?;
        }
        Ok(())
    }
}
