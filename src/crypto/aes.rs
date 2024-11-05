use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use rand::Rng;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn encrypt_file(file_path: &Path) -> Result<(Vec<u8>, Vec<u8>, std::path::PathBuf), Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let mut rng = rand::thread_rng();
    let mut key_bytes = [0u8; 32];
    rng.fill(&mut key_bytes);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let cipher = Aes256Gcm::new(key);
    let encrypted_data = cipher.encrypt(nonce, contents.as_ref())
        .map_err(|e| format!("Encryption failed: {}", e))?;
    let output_path = file_path.with_extension("encrypted");
    
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(&encrypted_data)?;

    Ok((key_bytes.to_vec(), nonce_bytes.to_vec(), output_path))
}

pub fn decrypt_file(
    file_path: &Path,
    key_str: &str,
    nonce_str: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let key_bytes = hex::decode(key_str.trim_start_matches('[').trim_end_matches(']')
        .split(',')
        .map(|s| s.trim())
        .collect::<String>())?;
    
    let nonce_bytes = nonce_str.trim_start_matches('[').trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let cipher = Aes256Gcm::new(key);
    let encrypted_data = fs::read(file_path)?;
    let decrypted_data = cipher.decrypt(nonce, encrypted_data.as_ref())
        .map_err(|e| format!("Decryption failed: {}", e))?;
    let output_path = file_path.with_extension("decrypted");
    
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(&decrypted_data)?;

    Ok(output_path)
}
