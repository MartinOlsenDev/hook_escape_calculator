use crate::lib::team::Team;
use iced::widget::{button, column, row, text};
use iced::{Element, Error, Settings, Theme};

pub fn run() -> iced::Result {
    iced::application("Hook Escape Calculator", update, view).run()
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct App {
    team: Team,
    chances: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    Base,
}

fn update(value: &mut App, message: Message) {
    todo!()
}

fn view(app: &App) -> Element<Message> {
    column![
        make_player_row(app, 0),
        make_player_row(app, 1),
        text(0).size(20),
        button("Test").on_press(Message::Base),
    ]
    .into()
}

fn make_player_row(app: &App, i: usize) -> Element<'static, Message> {
    row![text(format!("Player {i}")), text("perk here")].into()
}
