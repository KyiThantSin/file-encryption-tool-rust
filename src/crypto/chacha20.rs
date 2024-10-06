use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

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

pub fn encrypt_file<T: AsRef<Path>>(file_path: T) -> Result<(), std::io::Error> {
    let sizes = Sizes::new();
    let (key, nonce) = sizes.generate();

    // Open 
    let mut file = File::open(file_path.as_ref())?;
    let mut data = Vec::new(); // read into a Vec<u8>

    file.read_to_end(&mut data)?;

    // ChaCha20 cipher instance
    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());
    cipher.apply_keystream(&mut data);
    // println!("Encrypted Data: {:?}", data);
    
    std::fs::create_dir_all("testings")?; // create if it does not exist

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) 
        .open("testings/encrypted_file.txt")?;

    file.write_all(&data)?;
    println!("Encrypted data written to file.");

    Ok(())
}