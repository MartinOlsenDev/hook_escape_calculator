use super::{App, Message};
use iced::{window, Subscription};

impl App {
    fn which_close(&self, id: window::Id) -> Message {
        if id == self.calculator.0 {
            Message::ExitApp
        } else {
            Message::CloseHelp
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let main_id = self.calculator.0.clone();

        window::close_events().map(Message::CloseWindow)
    }
}
