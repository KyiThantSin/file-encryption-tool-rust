# Encora - Simple File Encryption Tool
Encora is a file encryption tool built with Rust, featuring a user-friendly interface created with the Iced library. It allows you to securely encrypt and decrypt files using either the AES or ChaCha20 algorithms. This tool is designed to be easy to use while providing strong data protection for anyone who needs to keep their files safe.

## Features
- **Supports AES and ChaCha20 Algorithms:** Choose between two secure encryption and decryption options.
- **User-Friendly GUI:** Built with Iced, enabling an intuitive and easy-to-use interface.
- **Clipboard Support:** Copy generated encryption keys and nonces to your clipboard for easy storage.
- **Save Encrypted/Decrypted Files:** Save your processed files in your desired location.

## Installation Instructions
### Prerequisites
To install Encora, ensure the following:

- **Rust:** Install the latest version from Rust's official site.
- **Iced Dependencies:** Dependencies vary by OS. Refer to the Iced setup guide for your system and version = "0.12.1".

### Cloning and Running the Project (Development)

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/KyiThantSin/file-encryption-tool-rust.git
    cd encora
    ```

2.  **Build the Project**:
    ```bash
    cargo build
    ```

3. **Run the Project**:
    ```bash
    cargo run
    ```
## Technology Stack
Encora leverages the following technologies and libraries:

- **Rust**
- **Iced**
- **AES and ChaCha20**
- **Copypasta**
  
## User Guide
#### Getting Started

Open Encora from your preferred method (compiled executable or running cargo run).
     
#### Encryption Process
- **Select Encryption Algorithm:** Choose either AES or ChaCha20.
- **Select File to Encrypt:** Browse to select the file you want to secure.
- **Click “Encrypt”:** Generates an encrypted file, along with a unique key and nonce.
- **Copy or Save Key/Nonce:** Copy the key and nonce to your clipboard for safe storage.
- **Save Encrypted File:** Save the output file to a location of your choice.
  
#### Decryption Process
- **Choose Algorithm:** Select the same algorithm (AES or ChaCha20) used to encrypt the file.
- **Input Key and Nonce:** Enter the key and nonce saved from the encryption process.
- **Select Encrypted File:** Browse to choose the file for decryption.
- **Click “Decrypt Now”:** The app will generate the decrypted file.
- **Save Decrypted File:** Download the decrypted file to your chosen location.

## Technical Overview
Encora’s technical implementation combines secure encryption libraries and a cross-platform GUI framework.

#### Encryption Algorithms
Encora uses two main algorithms:

- **AES:** Advanced Encryption Standard, an industry-standard symmetric encryption algorithm.
- **ChaCha20:** A secure and fast encryption algorithm suitable for high-speed and low-power applications.
Both algorithms generate unique encryption keys and nonces (initialization vectors) for each encryption session.

#### Structure of the Project
##### Core Encryption Logic:
- **crypto/chacha20.rs:** Contains ChaCha20 encryption and decryption functions.
- **crypto/aes.rs:** Contains AES encryption and decryption functions.
  
##### GUI (Graphical User Interface):
**main.rs:** Initializes the GUI using Iced. Manages UI state, file interactions, and connects to encryption logic.




