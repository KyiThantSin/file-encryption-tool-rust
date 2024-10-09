use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use rand::Rng;
use sha2::{Sha256, Digest};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use hex;
struct Sizes;

impl Sizes {
    fn new() -> Self {
        Self
    }

    // Random key and nonce
    fn generate(&self) -> ([u8; 32], [u8; 12]) {
        let mut key = [0u8; 32];
        let mut nonce = [0u8; 12];

        rand::thread_rng().fill(&mut key);
        rand::thread_rng().fill(&mut nonce);
        (key, nonce)
    }
}

pub fn encrypt_file<T: AsRef<Path>>(file_path: T) -> Result<(String, String), std::io::Error> {
    let sizes = Sizes::new();
    let (key, nonce) = sizes.generate();

    // Open the file
    let mut file = File::open(file_path.as_ref())?;
    let mut data = Vec::new(); // Read into a Vec<u8>
    file.read_to_end(&mut data)?;

    // ChaCha20 cipher instance
    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());
    cipher.apply_keystream(&mut data);

    // Create directory if it does not exist
    std::fs::create_dir_all("testings")?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) 
        .open("testings/encrypted_file.txt")?;

    file.write_all(&data)?;
    println!("Encrypted data written to file.");

    // Hash key and nonce using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(&key);
    let key_hash = hasher.finalize();
    
    let mut hasher_nonce = Sha256::new();
    hasher_nonce.update(&nonce);
    let nonce_hash = hasher_nonce.finalize();

    // Convert the hashed values to hexadecimal strings
    let key_hex = hex::encode(key_hash);
    let nonce_hex = hex::encode(nonce_hash);

    Ok((key_hex, nonce_hex))
}

pub fn decrypt_file<T: AsRef<Path>>(file_path: T) -> Result<(), std::io::Error>{
    let sizes = Sizes::new();
    let (key, nonce) = sizes.generate();

    let mut file = File::open(file_path.as_ref())?;
    let mut data: Vec<u8> = Vec::new();

    file.read_to_end(&mut data)?;

    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());
    cipher.apply_keystream(&mut data);

    std::fs::create_dir_all("testings")?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("testings/decrypted_file.txt")?;

    file.write_all(&data)?;
    println!("Decrypted data written to file.");

    Ok(())
}