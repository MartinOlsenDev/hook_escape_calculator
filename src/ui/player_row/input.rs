use arrayvec::{ArrayVec, ArrayString};
use iced::widget::{
    button, column, text, Column, row, Row, ComboBox, combo_box,
    Checkbox, checkbox
};
use iced::Element;
use derive_more::Display;
use strum::{IntoEnumIterator, EnumIter};
use derive_getters::Getters;

use crate::lib::player::PlayerData;
use crate::lib::loadout::Loadout as LoadoutData;
use crate::lib::perk::Tier;
use crate::lib::offering::Offering;

use super::super::Message;

// TODO: implement Display with proper whitespace

#[derive(Debug, Clone, Getters, Default)]
pub struct PlayerInputRow {
    loadout: Loadout,
    is_dead: bool
}

impl PlayerInputRow {
    pub fn view(&self) -> Element<Message> {
        row![
            self.loadout.view(),
            checkbox("is dead", self.is_dead)
                .on_toggle(|_| Message::Unimplemented)
        ].into()
    }
}

#[derive(Debug, Clone, Getters, Default)]
struct Loadout {
    data: LoadoutData,
    ui: LoadoutUI
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
pub enum PerkChoice {
    NA,
    One,
    Two,
    Three
}

impl From<Option<Tier>> for PerkChoice {
    fn from(value: Option<Tier>) -> Self {
        match value {
            None => Self::NA,
            Some(Tier::One) => Self::One,
            Some(Tier::Two) => Self::Two,
            Some(Tier::Three) => Self::Three
        }
    }
}

impl From<PerkChoice> for Option<Tier> {
    fn from(value: PerkChoice) -> Self {
        Some(match value {
            PerkChoice::NA => return None,
            PerkChoice::One => Tier::One,
            PerkChoice::Two => Tier::Two,
            PerkChoice::Three => Tier::Three
        })
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Display, EnumIter)]
pub enum OfferingChoice {
    NA,
    ChalkPouch,
    CreamPouch,
    SaltPouch,
    SaltStatuette,
    IvoryPouch,
    SaltyLips
}

impl From<Option<Offering>> for OfferingChoice {
    fn from(value: Option<Offering>) -> Self {
        match value {
            None => Self::NA,
            Some(Offering::ChalkPouch) => Self::ChalkPouch,
            Some(Offering::CreamPouch) => Self::CreamPouch,
            Some(Offering::IvoryPouch) => Self::IvoryPouch,
            Some(Offering::SaltPouch) => Self::SaltPouch,
            Some(Offering::SaltStatuette) => Self::SaltStatuette,
            Some(Offering::SaltyLips) => Self::SaltyLips
        }
    }
}

impl From<OfferingChoice> for Option<Offering> {
    fn from(value: OfferingChoice) -> Self {
        Some(match value {
            OfferingChoice::NA => return None,
            OfferingChoice::ChalkPouch => Offering::ChalkPouch,
            OfferingChoice::CreamPouch => Offering::CreamPouch,
            OfferingChoice::IvoryPouch => Offering::IvoryPouch,
            OfferingChoice::SaltPouch => Offering::SaltPouch,
            OfferingChoice::SaltStatuette => Offering::SaltStatuette,
            OfferingChoice::SaltyLips => Offering::SaltyLips
        })
    }
}

impl Loadout {
    fn view(&self) -> Element<Message> {
        row![
            combo_box(
                self.ui().slippery(),
                "",
                Some(&PerkChoice::from(
                    self.data()
                        .get_slippery()
                        .map(|x| x.tier().clone())
                )),
                |_| { println!("slippery combo"); Message::Unimplemented}
            ),
            combo_box(
                self.ui().uta(),
                "",
                Some(&PerkChoice::from(
                    self.data()
                        .get_uta()
                        .map(|x| x.tier().clone())
                )),
                |_| { println!("uta combo"); Message::Unimplemented}
            ),
            combo_box(
                self.ui().offering(),
                "",
                Some(&OfferingChoice::from(
                    self.data()
                        .get_offering()
                )),
                |_| { println!("offering combo"); Message::Unimplemented}
            )
        ].into()
    }
}

#[derive(Debug, Clone, Getters)]
struct LoadoutUI {
    slippery: combo_box::State<PerkChoice>,
    uta: combo_box::State<PerkChoice>,
    offering: combo_box::State<OfferingChoice>
}

impl Default for LoadoutUI {
    fn default() -> Self {
        Self {
            slippery: combo_box::State::new(PerkChoice::iter().collect()),
            uta: combo_box::State::new(PerkChoice::iter().collect()),
            offering: combo_box::State::new(OfferingChoice::iter().collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn NA_offering_display() {
        assert_eq!("NA", OfferingChoice::NA.to_string())
    }
    #[test]
    fn SaltPouch_offering_display() {
        assert_eq!("SaltPouch", OfferingChoice::SaltPouch.to_string())
    }
}