use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use rand::{rngs::OsRng, RngCore};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn encrypt_file(file_path: &Path) -> Result<(Vec<u8>, Vec<u8>, std::path::PathBuf), Box<dyn std::error::Error>> {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    let nonce = Nonce::from_slice(&nonce);

    let mut file = File::open(file_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    let encrypted_content = cipher
        .encrypt(nonce, content.as_ref())
        .map_err(|e| format!("Encryption failed: {}", e))?;

    let output_path = file_path.with_extension("encrypted");

    let mut output_file = File::create(&output_path)?;
    output_file.write_all(&encrypted_content)?;
    
    Ok((key.to_vec(), nonce.to_vec(), output_path))
}

pub fn decrypt_file(
    file_path: &Path,
    key_hex: &str,
    nonce_hex: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let key_bytes = hex::decode(key_hex.replace(['[', ']', ' '], ""))?;
    let nonce_bytes = hex::decode(nonce_hex.replace(['[', ']', ' '], ""))?;
    
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let mut file = File::open(file_path)?;
    let mut encrypted_content = Vec::new();
    file.read_to_end(&mut encrypted_content)?;

    let decrypted_content = cipher
        .decrypt(nonce, encrypted_content.as_ref())
        .map_err(|e| format!("Decryption failed: {}", e))?;

    let output_path = file_path.with_extension("decrypted");

    let mut output_file = File::create(&output_path)?;
    output_file.write_all(&decrypted_content)?;
    
    Ok(output_path)
}
