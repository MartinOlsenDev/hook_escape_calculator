use super::{App, Message};
use iced::{window, Subscription};

impl App {
    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::CloseWindow)
    }
}
