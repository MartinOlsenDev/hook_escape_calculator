use super::Message;
use iced::widget::{container, text, Column, Container};
use iced::Element;
use konst::string as ks;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const ABOUT_WIDTH: f32 = 380.;
const ABOUT_HEIGHT: f32 = 400.;

pub fn window_settings() -> iced::window::Settings {
    let size = iced::Size::new(ABOUT_WIDTH, ABOUT_HEIGHT);

    iced::window::Settings {
        resizable: false,
        size,
        ..iced::window::Settings::default()
    }
}

pub fn view() -> Element<'static, Message> {
    let mut c = Column::new();
    for s in TEXT {
        c = c.push(about_centered_container(s))
    }

    c.into()
}

const TEXT: [&str; 13] = [
    "Hook Calculator Copyright © 2025 Martin Olsen",
    ks::str_concat!(&["Version: ", VERSION]),
    "This program comes with ABSOLUTELY",
    "NO WARRANTY;",
    "",
    "This is free software, and you are welcome to",
    "redistribute it under the terms of the ",
    "GNU General Public License version 3",
    "or later.",
    "",
    "View the source code and license at:",
    "https://github.com/MartinOlsenDev",
    "/hook_escape_calculator",
];

fn about_centered_container(s: &str) -> Container<Message> {
    container(text(s).center()).center_x(ABOUT_WIDTH)
}
