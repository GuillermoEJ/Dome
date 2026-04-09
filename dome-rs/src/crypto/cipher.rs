/// AES-256 CFB mode encryption/decryption with zlib compression
/// 
/// Format: zlib(IV + AES_CFB_encrypted_data)

use aes::Aes256;
use cipher::{KeyIvInit, AsyncStreamCipher};
use cfb_mode::{Encryptor, Decryptor};
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use std::io::{Read, Write};
use anyhow::{Result, anyhow};

use super::derive_key;

type Aes256CfbEnc = Encryptor<Aes256>;
type Aes256CfbDec = Decryptor<Aes256>;

/// Encrypt data with AES-256 CFB mode and zlib compression
/// 
/// # Arguments
/// * `data` - Plain text bytes to encrypt
/// * `password` - Password for key derivation
/// 
/// # Returns
/// Compressed cipher bytes (zlib(IV + encrypted_data))
pub fn encrypt(data: &[u8], password: &str) -> Result<Vec<u8>> {
    // Derive key from password
    let key = derive_key(password)?;
    
    // Generate random IV
    let mut iv = [0u8; 16];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut iv);
    
    // Encrypt data with AES-256 CFB
    let mut cipher = Aes256CfbEnc::new_from_slices(&key, &iv)
        .map_err(|e| anyhow!("Failed to initialize cipher: {}", e))?;
    
    let mut encrypted = vec![0u8; data.len()];
    encrypted.copy_from_slice(data);
    cipher.encrypt(&mut encrypted);
    
    // Prepend IV to encrypted data
    let mut payload = iv.to_vec();
    payload.extend_from_slice(&encrypted);
    
    // Compress with zlib
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&payload)?;
    let compressed = encoder.finish()?;
    
    Ok(compressed)
}

/// Decrypt zlib-compressed data encrypted with AES-256 CFB mode
/// 
/// # Arguments
/// * `encrypted` - Compressed cipher bytes
/// * `password` - Password for key derivation
/// 
/// # Returns
/// Plain text bytes
pub fn decrypt(encrypted: &[u8], password: &str) -> Result<Vec<u8>> {
    // Decompress with zlib
    let mut decoder = GzDecoder::new(encrypted);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    
    if decompressed.len() < 16 {
        return Err(anyhow!("Invalid encrypted data: too short"));
    }
    
    // Extract IV and encrypted data
    let iv = &decompressed[0..16];
    let ciphertext = &decompressed[16..];
    
    // Derive key from password
    let key = derive_key(password)?;
    
    // Decrypt with AES-256 CFB
    let mut cipher = Aes256CfbDec::new_from_slices(&key, iv)
        .map_err(|e| anyhow!("Failed to initialize cipher: {}", e))?;
    
    let mut plaintext = ciphertext.to_vec();
    cipher.decrypt(&mut plaintext);
    
    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() -> Result<()> {
        let plaintext = b"Hello, DOME!";
        let password = "mysecretpassword";
        
        let encrypted = encrypt(plaintext, password)?;
        let decrypted = decrypt(&encrypted, password)?;
        
        assert_eq!(plaintext.to_vec(), decrypted);
        Ok(())
    }

    #[test]
    fn test_wrong_password() -> Result<()> {
        let plaintext = b"Secret data";
        let ciphertext = encrypt(plaintext, "password1")?;
        
        let result = decrypt(&ciphertext, "wrongpassword");
        // Decryption will succeed but produce garbage
        assert!(result.is_ok());
        assert_ne!(plaintext.to_vec(), result?);
        Ok(())
    }
}
