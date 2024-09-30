use iced::{
    alignment::{Horizontal, Vertical},
    widget::{column, text, container, Space},
    Length, Sandbox, Settings,
};

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp;

impl Sandbox for MyApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("File Encryption Tool")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<Self::Message> {
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
                        .width(Length::Fill)
                ]
            )
            .padding([10,50])
            .width(Length::Fill),

        ]
        .into()
    }
}
