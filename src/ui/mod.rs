use iced::widget::{column, Column, Text};
use iced::{Element, Error, Settings, Theme};

pub struct App {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    Base,
}
