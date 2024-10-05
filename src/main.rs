mod gui;
mod crypto;

use iced::{Sandbox, Settings};

fn main(){
    gui::MyApp::run(Settings::default());
}