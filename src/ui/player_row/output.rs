use arrayvec::{ArrayVec, ArrayString};
use iced::widget::{
    button, column, text, Column, row, Row, ComboBox, combo_box,
    Checkbox, checkbox
};
use iced::Element;
use derive_more::Display;

use super::super::Message;

#[derive(Debug, Clone)]
pub struct PlayerOddsDisplay {
    single: String,
    total: String
}

impl PlayerOddsDisplay {
    pub fn new<T: Into<String>>(s1: T, s2: T) -> Self {
        PlayerOddsDisplay {
            single: s1.into(),
            total: s2.into()
        }
    }

    pub fn view(&self) -> Element<Message> {
        row![
            text(&self.single),
            text(&self.total)
        ].into()
    }
}