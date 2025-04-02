mod api;
mod ui;

use iced::{Settings, window};
use ui::app::PasswordManagerApp;
use iced::Application;

fn main() -> iced::Result {
    PasswordManagerApp::run(Settings {
        window: window::Settings {
            size: (900, 600),
            resizable: true,
            decorations: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}