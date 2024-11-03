use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use hex;


pub fn encrypt_file<T: AsRef<Path>>(file_path: T) -> Result<(String, String, PathBuf), io::Error> {
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 12];
   
    rand::thread_rng().fill(&mut key);
    rand::thread_rng().fill(&mut nonce);

    let mut file = File::open(file_path.as_ref())?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());
    cipher.apply_keystream(&mut data);

    let original_name = file_path.as_ref()
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("encrypted_file");
    let output_name = format!("{}_encrypted.txt", original_name);
    let output_path = PathBuf::from("testings").join(output_name);

    std::fs::create_dir_all("testings")?;
    
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)?;

    file.write_all(&data)?;

    let key_hex = hex::encode(key);
    let nonce_hex = hex::encode(nonce);

    Ok((key_hex, nonce_hex, output_path))
}

pub fn decrypt_file<T: AsRef<Path>>(file_path: T, key_hex: &str, nonce_hex: &str) -> Result<PathBuf, io::Error> {
    let key = hex::decode(key_hex).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let nonce = hex::decode(nonce_hex).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    if key.len() != 32 || nonce.len() != 12 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid key or nonce length"));
    }

    let mut file = File::open(file_path.as_ref())?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;

    let mut cipher = ChaCha20::new(key.as_slice().into(), nonce.as_slice().into());
    cipher.apply_keystream(&mut data);

    let original_name = file_path.as_ref()
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("decrypted_file");
    let output_name = format!("{}_decrypted.txt", original_name);
    let output_path = PathBuf::from("testings").join(output_name);

    std::fs::create_dir_all("testings")?;
    
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)?;

    file.write_all(&data)?;

    Ok(output_path)
}