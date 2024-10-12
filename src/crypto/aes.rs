use aes::{Aes256, NewBlockCipher};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use generic_array::{GenericArray, ArrayLength};
use rand::Rng;
use std::fs::File;
use std::io::{self, Write, Read};
use std::path::PathBuf;

pub fn encrypt_aes(file_path: &PathBuf, key: &[u8]) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let cipher = Aes256::new_from_slice(key).expect("Failed to create cipher");

    // Generate a random IV
    let iv = rand::thread_rng().gen::<[u8; 16]>();
    let cipher = Cbc::<Aes256, Pkcs7>::new(cipher, GenericArray::from_slice(&iv));
    
    // Encrypt the data
    let encrypted_data = cipher.encrypt_vec(&buffer); // Use encrypt_vec to return Vec<u8>

    // Write the IV and encrypted data to a new file
    let encrypted_file_path = file_path.with_extension("enc");
    let mut encrypted_file = File::create(encrypted_file_path)?;
    encrypted_file.write_all(&iv)?;
    encrypted_file.write_all(&encrypted_data)?;

    Ok(())
}

pub fn decrypt_aes(file_path: &PathBuf, key: &[u8]) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Read IV
    let iv = &buffer[0..16];
    let data = &buffer[16..];

    let cipher = Aes256::new_from_slice(key).expect("Failed to create cipher");
    let cipher = Cbc::<Aes256, Pkcs7>::new(cipher, GenericArray::from_slice(iv));
    
    // Decrypt the data
    let decrypted_data = cipher.decrypt_vec(data).expect("Failed to decrypt data");

    // Write the decrypted data to a new file
    let decrypted_file_path = file_path.with_extension("dec");
    let mut decrypted_file = File::create(decrypted_file_path)?;
    decrypted_file.write_all(&decrypted_data)?;

    Ok(())
}
