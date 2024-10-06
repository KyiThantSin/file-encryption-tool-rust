use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

struct Sizes;

impl Sizes {
    fn new() -> Self {
        Self
    }
    // random key and nonce
    fn generate(&self) -> ([u8; 32], [u8; 12]) {
        let mut key = [0u8; 32]; 
        let mut nonce = [0u8; 12]; 

        rand::thread_rng().fill(&mut key); 
        rand::thread_rng().fill(&mut nonce); 
        (key, nonce)
    }
}

pub fn encrypt_file() -> io::Result<()> {
    let sizes = Sizes::new();
    let (key, nonce) = sizes.generate();

    let mut data = b"Hello, ChaCha20 encryption!".to_vec();

     // chacha 20 cipher instance
    let mut cipher = ChaCha20::new(&key.into(),&nonce.into());

    cipher.apply_keystream(&mut data);
    println!("Encrypted Data {:?}", data);
    let mut file = OpenOptions::new().write(true).create(true).open("testings/encrypted_file.txt")?;
    file.write_all(&data);
    println!("Encrypted data written to file.");

    Ok(())
}
