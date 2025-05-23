use iced::Element;
use iced::widget::{text, row};
use super::Message;

pub fn view() -> Element<'static, Message> {
    row![
        text("This is an about message."),
        text("Second row about message.")
    ].into()
}