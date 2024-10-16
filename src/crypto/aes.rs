use aes_gcm::{Aes256Gcm, Key, Nonce}; // Add this import for AES-256 GCM
use std::path::Path;
use aes_gcm::KeyInit;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use rand::Rng;
use aes_gcm::aead::Aead;

pub fn aes_encrypt_file(input_path: &Path, key: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let mut file = File::open(input_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Ensure that the key is 32 bytes long (for AES-256)
    let key_bytes = key.as_bytes();
    if key_bytes.len() != 32 {
        return Err("Key must be 32 bytes long for AES-256.".into());
    }

    let key = Key::<Aes256Gcm>::from_slice(key_bytes); // Specify the type here
    let cipher = Aes256Gcm::new(key);

    // Generate a random 96-bit nonce
    let mut rng = rand::thread_rng();
    let nonce: [u8; 12] = rng.gen();
    let nonce = Nonce::from_slice(&nonce);

    let encrypted_contents = cipher.encrypt(nonce, contents.as_ref())
        .map_err(|e| format!("Encryption error: {}", e))?;

    let output_path = input_path.with_extension("aes");
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(nonce)?;  // Write the nonce first
    output_file.write_all(&encrypted_contents)?;

    Ok(output_path)
}
pub fn aes_decrypt_file(input_path: &Path, key: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let mut file = File::open(input_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    if contents.len() < 12 {
        return Err("File is too short to contain a valid nonce".into());
    }

    let (nonce, ciphertext) = contents.split_at(12);

    // Ensure that the key is 32 bytes long (for AES-256)
    let key_bytes = key.as_bytes();
    if key_bytes.len() != 32 {
        return Err("Key must be 32 bytes long for AES-256.".into());
    }

    let key = Key::<Aes256Gcm>::from_slice(key_bytes); // Specify the type here
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);

    let decrypted_contents = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption error: {}", e))?;

    let output_path = input_path.with_extension("decrypted");
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(&decrypted_contents)?;

    Ok(output_path)
}
