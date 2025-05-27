use super::{App, Message};
use iced::{Subscription, window};

impl App {
    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::CloseWindow)
    }
}
