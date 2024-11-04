use crate::crypto::chacha20::{decrypt_file, encrypt_file};
use crate::crypto::aes::{aes_encrypt_file, aes_decrypt_file};
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, pick_list, row, text, text_input, Space},
    Element, Length, Sandbox, theme
};
use rfd::FileDialog;
use copypasta::{ClipboardContext, ClipboardProvider};

#[derive(Debug, Clone)]
pub enum MyAppMessage {
    AlgorithmSelected(Algorithms),
    StartEncryption,
    StartDecryption,
    FileSelected(Option<std::path::PathBuf>),
    OpenFileDialog,
    KeyInputChanged(String),
    NonceInputChanged(String),
    Encrypt,
    Decrypt,
    BackToMain,
    CopyKey,
    CopyNonce,
    DownloadFile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithms {
    AES,
    ChaCha20,
}

impl Algorithms {
    const ALL: [Algorithms; 2] = [Algorithms::AES, Algorithms::ChaCha20];
}

impl std::fmt::Display for Algorithms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithms::AES => write!(f, "AES"),
            Algorithms::ChaCha20 => write!(f, "ChaCha20"),
        }
    }
}

pub struct MyApp {
    pub selected_algorithm: Option<Algorithms>,
    pub encryption_status: String,
    pub decryption_status: String,
    pub selected_file: Option<std::path::PathBuf>,
    pub key: String,
    pub nonce: String,
    pub show_key_nonce_input: bool,
    pub copy_status: String,
    pub processed_file: Option<std::path::PathBuf>,
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        Self {
            selected_algorithm: None,
            encryption_status: "".into(),
            decryption_status: "".into(),
            copy_status:"".into(),
            selected_file: None,
            key: "".into(),
            nonce: "".into(),
            show_key_nonce_input: false,
            processed_file: None,
        }
    }

    fn title(&self) -> String {
        String::from("File Encryption Tool")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MyAppMessage::AlgorithmSelected(algorithm) => {
                self.selected_algorithm = Some(algorithm);
            }
            MyAppMessage::StartEncryption => {
                self.show_key_nonce_input = true;
                self.encryption_status = "Ready to encrypt".into();
                self.decryption_status = "".into();
            }
            MyAppMessage::StartDecryption => {
                self.show_key_nonce_input = true;
                self.decryption_status = "Ready to decrypt".into();
                self.encryption_status = "".into();
            }
            MyAppMessage::FileSelected(file_path) => {
                self.selected_file = file_path;
            }
            MyAppMessage::OpenFileDialog => {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.selected_file = Some(path);
                }
            }
            MyAppMessage::KeyInputChanged(new_key) => {
                self.key = new_key;
            }
            MyAppMessage::NonceInputChanged(new_nonce) => {
                self.nonce = new_nonce;
            }
            MyAppMessage::Encrypt => {
                if let Some(selected_file) = &self.selected_file {
                    if let Some(algorithm) = self.selected_algorithm {
                        match algorithm {
                            Algorithms::ChaCha20 => {
                                if self.key.is_empty() || self.nonce.is_empty() {
                                    self.encryption_status = "Please provide both key and nonce for ChaCha20".into();
                                    return;
                                }
                                match encrypt_file(selected_file, &self.key, &self.nonce) {
                                    Ok(encrypted_file_path) => {
                                        self.encryption_status = format!("File encrypted successfully with ChaCha20. Saved to: {}", encrypted_file_path.display());
                                        self.processed_file = Some(encrypted_file_path);
                                    }
                                    Err(e) => {
                                        self.encryption_status = format!("Error encrypting file with ChaCha20: {}", e);
                                    }
                                }
                            },
                            Algorithms::AES => {
                                if self.key.is_empty() {
                                    self.encryption_status = "Please provide a key for AES".into();
                                    return;
                                }
                                match aes_encrypt_file(selected_file, &self.key) {
                                    Ok(encrypted_file_path) => {
                                        self.encryption_status = format!("File encrypted successfully with AES. Saved to: {}", encrypted_file_path.display());
                                        self.processed_file = Some(encrypted_file_path);
                                    }
                                    Err(e) => {
                                        self.encryption_status = format!("Error encrypting file with AES: {}", e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            MyAppMessage::Decrypt => {
                if let Some(selected_file) = &self.selected_file {
                    if let Some(algorithm) = self.selected_algorithm {
                        match algorithm {
                            Algorithms::ChaCha20 => {
                                if self.key.is_empty() || self.nonce.is_empty() {
                                    self.decryption_status = "Please provide both key and nonce for ChaCha20".into();
                                    return;
                                }
                                match decrypt_file(selected_file, &self.key, &self.nonce) {
                                    Ok(decrypted_file_path) => {
                                        self.decryption_status = format!("File decrypted successfully with ChaCha20. Saved to: {}", decrypted_file_path.display());
                                        self.processed_file = Some(decrypted_file_path);
                                    }
                                    Err(e) => {
                                        self.decryption_status = format!("Error decrypting file with ChaCha20: {}", e);
                                    }
                                }
                            },
                            Algorithms::AES => {
                                if self.key.is_empty() {
                                    self.decryption_status = "Please provide a key for AES".into();
                                    return;
                                }
                                match aes_decrypt_file(selected_file, &self.key) {
                                    Ok(decrypted_file_path) => {
                                        self.decryption_status = format!("File decrypted successfully with AES. Saved to: {}", decrypted_file_path.display());
                                        self.processed_file = Some(decrypted_file_path);
                                    }
                                    Err(e) => {
                                        self.decryption_status = format!("Error decrypting file with AES: {}", e);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            MyAppMessage::BackToMain => {
                self.show_key_nonce_input = false;
                self.encryption_status = "".into();
                self.decryption_status = "".into();
            }
            MyAppMessage::CopyKey => {
                let mut ctx = ClipboardContext::new().unwrap();
                ctx.set_contents(self.key.clone()).unwrap();
                self.copy_status = "Key copied to clipboard".into();
            }
            MyAppMessage::CopyNonce => {
                let mut ctx = ClipboardContext::new().unwrap();
                ctx.set_contents(self.nonce.clone()).unwrap();
                self.copy_status = "Nonce copied to clipboard".into();
            }
            MyAppMessage::DownloadFile => {
                if let Some(file_path) = &self.processed_file {
                    if let Some(save_path) = FileDialog::new()
                        .set_file_name(file_path.file_name().unwrap().to_str().unwrap())
                        .save_file() {
                        if let Err(e) = std::fs::copy(file_path, save_path) {
                            self.copy_status = format!("Error saving file: {}", e);
                        } else {
                            self.copy_status = "File saved successfully".into();
                            self.processed_file = None; // Reset processed_file after successful download
                        }
                    }
                }
            }
        }
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            container(column![
                text("Encora")
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Left)
                    .vertical_alignment(Vertical::Center)
                    .size(28)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb(0.0,0.5,0.9))),
                Space::with_height(10),
                text("Your Trusted File Encryption Tool").width(Length::Fill)
            ])
            .padding([50, 50])
            .width(Length::Fill),
            container(column![
                row![
                    text("Encora supports two encryption algorithms: ChaCha20 and AES.")
                        .width(Length::Shrink),
                    Space::with_width(Length::Fill),
                    pick_list(
                        &Algorithms::ALL[..],
                        self.selected_algorithm,
                        MyAppMessage::AlgorithmSelected
                    )
                    .placeholder("Select an algorithm")
                    .width(Length::Shrink),
                ]
                .width(Length::Fill),
                Space::with_height(20),
                text("Please choose one to proceed with encryption.").width(Length::Fill),
                text(&self.encryption_status)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
            ])
            .padding([10, 50])
            .width(Length::Fill),
            Space::with_height(30),
            container(
                column![
                    if let Some(selected_file) = &self.selected_file {
                        button(text(format!("Selected file: {}", selected_file.display())))
                            .on_press(MyAppMessage::OpenFileDialog)
                            .padding(15)
                            .style(theme::Button::Secondary)
                    } else {
                        button(text("Select a file..."))
                            .on_press(MyAppMessage::OpenFileDialog)
                            .padding(15)
                            .width(Length::Fixed(900.0))
                            .style(theme::Button::Secondary)
                    },
                    Space::with_height(30),
                    if !self.show_key_nonce_input {
                        container(
                            row![
                                button(text("Encrypt"))
                                    .on_press(MyAppMessage::StartEncryption)
                                    .padding(10),
                                Space::with_width(20),
                                button(text("Decrypt"))
                                    .on_press(MyAppMessage::StartDecryption)
                                    .padding(10),
                            ]
                            .align_items(iced::Alignment::Center)
                        )
                    } else {
                        container(
                            column![
                                text("Key:").width(Length::Shrink).horizontal_alignment(Horizontal::Left),
                                Space::with_height(10),
                                text_input("Enter Key", &self.key)
                                    .on_input(MyAppMessage::KeyInputChanged)
                                    .padding(10)
                                    .width(Length::Fill),
                                Space::with_height(20),
                                if self.selected_algorithm == Some(Algorithms::ChaCha20) {
                                    column![
                                        text("Nonce:").width(Length::Shrink).horizontal_alignment(Horizontal::Left),
                                        Space::with_height(10),
                                        text_input("Enter Nonce", &self.nonce)
                                            .on_input(MyAppMessage::NonceInputChanged)
                                            .padding(10)
                                            .width(Length::Fill),
                                        Space::with_height(20),
                                    ]
                                } else {
                                    column![]
                                },
                                row![
                                    button(text(if self.encryption_status.contains("encrypt") { "Encrypt" } else { "Decrypt" }))
                                        .on_press(if self.encryption_status.contains("encrypt") { MyAppMessage::Encrypt } else { MyAppMessage::Decrypt })
                                        .padding(10),
                                    Space::with_width(20),
                                    button(text("Back"))
                                        .on_press(MyAppMessage::BackToMain)
                                        .padding(10),
                                ]
                                .spacing(10)
                                .align_items(iced::Alignment::Center)
                            ].padding([50, 50])
                        )
                    },

                    Space::with_height(10),
                    text(&self.encryption_status)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center)
                        .size(15)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.2, 0.8, 0.2))),
                    text(&self.decryption_status)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center)
                        .size(15)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.2, 0.8, 0.2))),
                    text(&self.copy_status)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center)
                        .size(15)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.0,0.5,0.9))),
                    if self.processed_file.is_some() {
                        container(
                            button(text("Download Processed File"))
                                .on_press(MyAppMessage::DownloadFile)
                                .padding(10)
                        )
                    } else {
                        container(Space::with_height(0))
                    },
                ]
                .align_items(iced::Alignment::Center)
            )
            .padding(20)
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill),
        ]
        .into()
    }
}
