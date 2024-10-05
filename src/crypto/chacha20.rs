use chacha20::{ChaCha20, Key, Nonce};
use rand::{rngs::OsRng, RngCore};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
