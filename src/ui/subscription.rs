use iced::{Subscription, window};

use super::{App, Message};

impl App {
    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::CloseWindow)
    }
}
