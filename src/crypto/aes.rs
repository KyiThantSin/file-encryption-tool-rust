use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use hex;
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub fn aes_encrypt_file<T: AsRef<Path>>(
    file_path: T,
) -> Result<(String, String, PathBuf), io::Error> {
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill(&mut key);
    rand::thread_rng().fill(&mut nonce);

    let mut file = File::open(file_path.as_ref())?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce);
    let encrypted_data = cipher
        .encrypt(nonce, data.as_ref())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Encryption error: {}", e)))?;

    let original_name = file_path
        .as_ref()
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("encrypted_file");
    let extension = file_path
        .as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    let output_name = if extension.is_empty() {
        format!("{}_encrypted", original_name)
    } else {
        format!("{}_encrypted.{}", original_name, extension)
    };
    let output_path = PathBuf::from("testings").join(output_name);
    std::fs::create_dir_all("testings")?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)?;
    file.write_all(&nonce)?;
    file.write_all(&encrypted_data)?;

    let key_hex = hex::encode(key.as_slice());
    let nonce_hex = hex::encode(nonce.as_slice());

    Ok((key_hex, nonce_hex, output_path))
}

pub fn aes_decrypt_file<T: AsRef<Path>>(
    file_path: T,
    key_hex: &str,
    nonce_hex: &str,
) -> Result<PathBuf, io::Error> {
    let key = hex::decode(key_hex).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let nonce =
        hex::decode(nonce_hex).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    if key.len() != 32 || nonce.len() != 12 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid key or nonce length",
        ));
    }

    let mut file = File::open(file_path.as_ref())?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let (file_nonce, encrypted_data) = data.split_at(12);

    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(file_nonce);
    let decrypted_data = cipher
        .decrypt(nonce, encrypted_data)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Decryption error: {}", e)))?;

    let original_name = file_path
        .as_ref()
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("decrypted_file");
    let extension = file_path
        .as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    let output_name = if extension.is_empty() {
        format!("{}_decrypted", original_name)
    } else {
        format!("{}_decrypted.{}", original_name, extension)
    };
    let output_path = PathBuf::from("testings").join(output_name);
    std::fs::create_dir_all("testings")?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)?;
    file.write_all(&decrypted_data)?;

    Ok(output_path)
}
