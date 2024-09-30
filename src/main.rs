use iced::{
    alignment::{Horizontal, Vertical},
    widget::{column, text, container, Space, pick_list},
    Length, Sandbox, Settings, Element, Theme,
};

#[derive(Debug, Clone)]
enum MyAppMessage {
    DoNothing,
    AlgorithmSelected(Algorithms),
    Open10,
    Close11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Algorithms {
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

struct MyApp {
    selected_algorithm: Option<Algorithms>,
    info_10: String,
    info_11: String,
}

impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
         Self { 
             selected_algorithm: None, 
             info_10: "".into(),
             info_11: "".into(),
         }
    }

    fn title(&self) -> String {
        String::from("File Encryption Tool")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            MyAppMessage::DoNothing => {}
            MyAppMessage::AlgorithmSelected(algorithm) => {
                self.selected_algorithm = Some(algorithm);
            }
            MyAppMessage::Open10 => self.info_10 = "Open".into(),
            MyAppMessage::Close11 => self.info_11 = "Close".into(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            container(
                column![
                    text("Encora")
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Left)
                        .vertical_alignment(Vertical::Center),

                    Space::with_height(10),

                    text("Your Trusted File Encryption Tool")
                        .width(Length::Fill)
                ]
            )
            .padding([50, 50])
            .width(Length::Fill),

            container(
                column![
                    text("Encora supports two encryption algorithms: ChaCha20 and AES.")
                        .width(Length::Fill),
                    Space::with_height(9),
                    text("Please choose one to proceed with encryption.")
                        .width(Length::Fill),

                    // selecting algorithm
                    pick_list(
                        &Algorithms::ALL[..], 
                        self.selected_algorithm,  // current selected algorithm
                        MyAppMessage::AlgorithmSelected 
                    )
                    .placeholder("Select an algorithm")
                    .width(Length::Shrink),

                    text(
                        self.selected_algorithm
                            .map_or("No algorithm selected".to_string(), |alg| format!("Selected: {}", alg))
                    )
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
                ]
            )
            .padding([10,50])
            .width(Length::Fill),
        ]
        .into()
    }
}

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}