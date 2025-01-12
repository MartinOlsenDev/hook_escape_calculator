use crate::lib::team::Team;
use iced::widget::{column, Column, Text};
use iced::{Element, Error, Settings, Theme};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct App {
    team: Team,
    chances: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    Base,
}
