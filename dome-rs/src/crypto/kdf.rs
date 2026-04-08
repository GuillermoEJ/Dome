/// Key derivation using PBKDF2-SHA256
/// 
/// Generates 256-bit (32-byte) keys from passwords using PBKDF2

use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use anyhow::{Result, anyhow};

use super::{SALT, KEY_LENGTH};

// Standard iteration count (adjust for security vs speed tradeoff)
const ITERATIONS: u32 = 100_000;

/// Derive a 256-bit key from a password using PBKDF2-SHA256
/// 
/// # Arguments
/// * `password` - The password to derive a key from
/// 
/// # Returns
/// The derived 256-bit key
pub fn derive_key(password: &str) -> Result<Vec<u8>> {
    let mut key = [0u8; KEY_LENGTH];
    
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        SALT,
        ITERATIONS,
        &mut key,
    );
    
    Ok(key.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_key_derivation() -> Result<()> {
        let password = "testpassword";
        
        let key1 = derive_key(password)?;
        let key2 = derive_key(password)?;
        
        assert_eq!(key1, key2);
        assert_eq!(key1.len(), KEY_LENGTH);
        Ok(())
    }

    #[test]
    fn test_different_passwords_different_keys() -> Result<()> {
        let key1 = derive_key("password1")?;
        let key2 = derive_key("password2")?;
        
        assert_ne!(key1, key2);
        Ok(())
    }
}
