use crate::crypto::chacha20::{decrypt_file, encrypt_file};
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, pick_list, row, text, text_input, Space},
    Element, Length, Sandbox,
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
    Decrypt,
    BackToMain,
    CopyKey,
    CopyNonce
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
            MyAppMessage::FileSelected(file_path) => {
                self.selected_file = file_path;
                self.encryption_status = String::new();
                self.decryption_status = String::new();
            }
            MyAppMessage::OpenFileDialog => {
                if let Some(path) = FileDialog::new().pick_file() {
                    self.update(MyAppMessage::FileSelected(Some(path)));
                } else {
                    self.update(MyAppMessage::FileSelected(None));
                }
            }
            MyAppMessage::StartEncryption => {
                self.encryption_status = "Encryption started".into();
                if let Some(selected_file) = &self.selected_file {
                    if let Some(algorithm) = self.selected_algorithm {
                        match algorithm {
                            Algorithms::ChaCha20 => match encrypt_file(selected_file) {
                                Ok((key, nonce)) => {
                                    self.key = format!("{:x?}", key);
                                    self.nonce = format!("{:?}", nonce);
                                    self.encryption_status = format!("File encrypted successfully");
                                }
                                Err(e) => {
                                    self.encryption_status =
                                        format!("Error encrypting file: {}", e);
                                }
                            },
                            Algorithms::AES => {
                                // AES encryption logic
                            }
                        }
                    }
                }
            }
            MyAppMessage::CopyKey => {
                let mut clipboard = ClipboardContext::new().unwrap();
                clipboard.set_contents(self.key.clone()).unwrap();
                self.copy_status = "Key copied to clipboard!".into();
            }
            MyAppMessage::CopyNonce => {
                let mut clipboard = ClipboardContext::new().unwrap();
                clipboard.set_contents(self.nonce.clone()).unwrap();
                self.copy_status = "Nonce copied to clipboard!".into();
            }
            MyAppMessage::BackToMain => {
                // show the main page
                self.show_key_nonce_input = false;
                self.encryption_status = String::new();
                self.decryption_status = String::new();
                self.key = String::new();
                self.nonce = String::new();
                self.selected_file = None;
                self.copy_status = String::new();
            }
            MyAppMessage::StartDecryption => {
                // Show input fields for key and nonce
                self.show_key_nonce_input = true;
                self.encryption_status = String::new();
                self.key = String::new();
                self.nonce = String::new();
                self.selected_file = None;
                self.copy_status = String::new();
            }
            MyAppMessage::KeyInputChanged(key) => {
                self.key = key;
            }
            MyAppMessage::NonceInputChanged(nonce) => {
                self.nonce = nonce;
            }
            MyAppMessage::Decrypt => {
                // Start decryption when both key and nonce are provided
                if self.key.is_empty() || self.nonce.is_empty() {
                    self.decryption_status = "Please provide both key and nonce to decrypt".into();
                } else {
                    if let Some(selected_file) = &self.selected_file {
                        if let Some(algorithm) = self.selected_algorithm {
                            match algorithm {
                                Algorithms::ChaCha20 => {
                                    if let Err(e) = decrypt_file(selected_file, &self.key, &self.nonce) {
                                        self.decryption_status =
                                            format!("Error decrypting file: {}", e);
                                    } else {
                                        self.decryption_status = format!(
                                            "File decrypted successfully: {:?}",
                                            selected_file
                                        );
                                        self.show_key_nonce_input = false; // Hide input fields after successful decryption
                                    }
                                }
                                Algorithms::AES => {
                                    // AES decryption logic
                                }
                            }
                        }
                    }
                }
            }
        }
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
                        self.selected_algorithm, // current selected algorithm
                        MyAppMessage::AlgorithmSelected
                    )
                    .placeholder("Select an algorithm")
                    .width(Length::Shrink),
                ]
                .width(Length::Fill),
                Space::with_height(9),
                text("Please choose one to proceed with encryption.").width(Length::Fill),
                text(
                    self.selected_algorithm
                        .map_or("No algorithm selected".to_string(), |_| "".to_string())
                )
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
            ])
            .padding([10, 50])
            .width(Length::Fill),
            container(
                column![
                    button(text("Select a file..."))
                        .on_press(MyAppMessage::OpenFileDialog)
                        .padding(15),
                    Space::with_height(10),
                    self.selected_file
                        .as_ref()
                        .map_or_else(|| text("No file selected"), |_| text("")),
                    Space::with_height(20),
                    
                    // Only show the Encrypt and Decrypt buttons if key and nonce input fields are not shown
                    if !self.show_key_nonce_input {
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
                    } else {
                        row![]
                    },
    
                        if self.encryption_status == "File encrypted successfully" {
                            container(column![
                                column![
                                    Space::with_height(20),
            
                                    // Display Key and Nonce
                                    text("Encryption Details").size(22).style(iced::theme::Text::Color(iced::Color::from_rgb(0.0,0.5,0.9))),
                                    Space::with_height(10),
                                        
                                    row![
                                        text("Key:").width(Length::Shrink).style(iced::theme::Text::Color(iced::Color::from_rgb(0.0,0.5,0.9))),
                                        text(&self.key)
                                            .width(Length::Fill)
                                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                                        button("Copy").on_press(MyAppMessage::CopyKey) 
                                    ]
                                    .align_items(iced::Alignment::Center),
                                    Space::with_height(10),
                                    row![
                                        text("Nonce:").width(Length::Shrink).style(iced::theme::Text::Color(iced::Color::from_rgb(0.0,0.5,0.9))),
                                        text(&self.nonce)
                                            .width(Length::Fill)
                                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                                        button("Copy").on_press(MyAppMessage::CopyNonce) 
                                    ],
                                    Space::with_height(20),
                                ]
                                .align_items(iced::Alignment::Center),
                                text("Please save the key and nonce somewhere safe in order to decrypt the file"),
                            ])
                            .width(Length::Fill)
                            .padding([50, 50])
                        } else {
                            container(column![])
                        },
    
                    // Show key and nonce input fields when decrypt button is clicked
                    if self.show_key_nonce_input {
                        column![
                            text("Key:").width(Length::Shrink).horizontal_alignment(Horizontal::Left),
                            Space::with_height(10),
                            text_input("Enter Key", &self.key)
                                .on_input(MyAppMessage::KeyInputChanged)
                                .padding(10)
                                .width(Length::Fill),
                            Space::with_height(20),
                            text("Nonce:").width(Length::Shrink).horizontal_alignment(Horizontal::Left),
                            Space::with_height(10),
                            text_input("Enter Nonce", &self.nonce)
                                .on_input(MyAppMessage::NonceInputChanged)
                                .padding(10)
                                .width(Length::Fill),
                            Space::with_height(20),
                            row![
                                button(text("Decrypt Now"))
                                    .on_press(MyAppMessage::Decrypt)
                                    .padding(10),
                                Space::with_width(20),
                                button(text("Back"))
                                    .on_press(MyAppMessage::BackToMain)
                                    .padding(10),
                            ]
                            .spacing(10)
                            .align_items(iced::Alignment::Center)
                        ]
                    } else {
                        column![]
                    },
    
                    // Show encryption or decryption status
                    Space::with_height(10),
                    text(&self.encryption_status)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center)
                        .size(20)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.2, 0.8, 0.2))),
                    text(&self.decryption_status)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center)
                        .size(20)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.2, 0.8, 0.2))),
                    text(&self.copy_status)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center)
                        .size(15)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.0,0.5,0.9))),
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
