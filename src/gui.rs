use crate::crypto::chacha20::{encrypt_file, decrypt_file};

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, pick_list, row, text, Space},
    Element, Length, Sandbox,
};
use rfd::FileDialog;

#[derive(Debug, Clone)]
pub enum MyAppMessage {
    AlgorithmSelected(Algorithms),
    StartEncryption,
    StopDecryption,
    FileSelected(Option<std::path::PathBuf>),
    OpenFileDialog,
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
    pub file_content: String,
    pub key: String,
    pub nonce: String,
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        Self {
            selected_algorithm: None,
            encryption_status: "".into(),
            decryption_status: "".into(),
            selected_file: None,
            file_content: "".into(),
            key: "".into(),
            nonce: "".into(),
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
                            Algorithms::ChaCha20 => {
                                match encrypt_file(selected_file) {
                                    Ok((key, nonce)) => {
                                        self.key = format!("{:x?}", key);
                                        self.nonce = format!("{:?}", nonce);
                                        self.encryption_status =
                                            format!("File encrypted successfully");
                                    }
                                    Err(e) => {
                                        self.encryption_status =
                                            format!("Error encrypting file: {}", e);
                                    }
                                }
                            }
                            Algorithms::AES => {
                                // AES encryption logic
                            }
                        }
                    }
                }
            }
            MyAppMessage::StopDecryption => {
                self.encryption_status = "Decrypted started".into();
                if let Some(selected_file) = &self.selected_file {
                    if let Some(algorithm) = self.selected_algorithm {
                        match algorithm {
                            Algorithms::ChaCha20 => {
                                if let Err(e) = decrypt_file(selected_file) {
                                    self.encryption_status =
                                        format!("Error decrypting file: {}", e);
                                } else {
                                    self.encryption_status = format!(
                                        "File decrypted successfully: {:?}",
                                        selected_file
                                    );
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

    fn view(&self) -> Element<Self::Message> {
        column![
            container(column![
                text("Encora")
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Left)
                    .vertical_alignment(Vertical::Center),
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
                    row![
                        button(text("Encrypt"))
                            .on_press(MyAppMessage::StartEncryption)
                            .padding(10),
                        Space::with_width(20),
                        button(text("Decrypt"))
                            .on_press(MyAppMessage::StopDecryption)
                            .padding(10),
                    ]
                    .align_items(iced::Alignment::Center),
                    text(&self.file_content)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center),
                    
                    // the encryption details if encryption was successful
                    if self.encryption_status == "File encrypted successfully" {
                        container(column![
                            column![
                                text(&self.encryption_status)
                                    .width(Length::Fill)
                                    .horizontal_alignment(Horizontal::Center),
                                Space::with_height(20),
    
                                // Display Key and Nonce
                                text("Encryption Details"),
                                Space::with_height(10),
                                
                                row![
                                    text("Key:").width(Length::Shrink),
                                    text(&self.key)
                                        .width(Length::Fill)
                                        .horizontal_alignment(Horizontal::Center),
                                ]
                                .align_items(iced::Alignment::Center),
                                Space::with_height(10),
                                row![
                                    text("Nonce:").width(Length::Shrink),
                                    text(&self.nonce)
                                        .width(Length::Fill)
                                        .horizontal_alignment(Horizontal::Center)
                                ],
                                Space::with_height(20),
                            ]
                            .align_items(iced::Alignment::Center),
                            text("Please save the key and nonce somewhere else in order to decrypt the file"),
                        ])
                        .width(Length::Fill)
                        .padding([50, 50])
                    } else {
                        container(
                            column![]
                        )
                    }
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
