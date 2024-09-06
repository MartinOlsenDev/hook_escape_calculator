use iced::widget::{column, Column, Text};
use iced::{Element, Error, Sandbox, Settings, Theme};

pub struct App {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    Base,
}

impl Sandbox for App {
    type Message = ();

    fn new() -> Self {
        todo!()
    }

    fn title(&self) -> String {
        "Dead by Daylight Hook Calculator".into()
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        todo!()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
