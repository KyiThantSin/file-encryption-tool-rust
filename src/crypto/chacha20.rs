// chacha20.rs
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce}; 
use chacha20poly1305::aead::{Aead, KeyInit};  

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;  

pub fn encrypt_file(input_path: &Path, key: &str, nonce: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {    
    let mut file = File::open(input_path)?;    
    let mut contents = Vec::new();    
    file.read_to_end(&mut contents)?;     

    let key = Key::from_slice(key.as_bytes());    
    let nonce = Nonce::from_slice(nonce.as_bytes());    
    let cipher = ChaCha20Poly1305::new(key);     

    let encrypted_contents = cipher.encrypt(nonce, contents.as_ref())        
        .map_err(|e| format!("Encryption error: {}", e))?;     

    let output_path = input_path.with_extension("chacha20");    
    let mut output_file = File::create(&output_path)?;    
    output_file.write_all(&encrypted_contents)?;     

    Ok(output_path) 
}  

pub fn decrypt_file(input_path: &Path, key: &str, nonce: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {    
    let mut file = File::open(input_path)?;    
    let mut contents = Vec::new();    
    file.read_to_end(&mut contents)?;     

    let key = Key::from_slice(key.as_bytes());    
    let nonce = Nonce::from_slice(nonce.as_bytes());    
    let cipher = ChaCha20Poly1305::new(key);     

    let decrypted_contents = cipher.decrypt(nonce, contents.as_ref())        
        .map_err(|e| format!("Decryption error: {}", e))?;     

    let output_path = input_path.with_extension("decrypted");    
    let mut output_file = File::create(&output_path)?;    
    output_file.write_all(&decrypted_contents)?;     

    Ok(output_path) 
}
