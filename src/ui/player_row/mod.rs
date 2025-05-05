pub mod input;
pub mod output;
use input::*;
use output::*;
use super::Message;

use iced::widget::{Row, row};
use iced::Element;

pub mod reexports {
    pub use super::input;
    pub use super::output;
}

#[derive(Debug, Clone)]
pub struct PlayerRow {
    interactables: PlayerInputRow,
    odds: PlayerOddsDisplay
}

impl PlayerRow {
    pub fn view(&self) -> Element<Message> {
        row![
            self.interactables.view(),
            self.odds.view()
        ].into()
    }
}

impl Default for PlayerRow {
    fn default() -> Self {
        let interactables = PlayerInputRow::default();
        let odds = PlayerOddsDisplay::new("tmp", "tmp");

        Self {
            interactables,
            odds
        }
    }
}