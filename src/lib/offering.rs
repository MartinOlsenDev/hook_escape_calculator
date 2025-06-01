use nutype::nutype;
use strum::{EnumIter, IntoEnumIterator};

use crate::constants::offering_luck as k;

use super::luck_record::{LoadoutLuckRecord, Luck};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Offering {
    ChalkPouch,
    CreamPouch,
    SaltPouch,
    SaltStatuette,
    IvoryPouch,
    SaltyLips,
}
#[nutype(
    derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, AsRef, Deref),
    default = None
)]
pub struct OfferingSlot(Option<Offering>);

impl From<&Offering> for LoadoutLuckRecord {
    fn from(offering: &Offering) -> Self {
        if offering.luck_is_personal() {
            LoadoutLuckRecord::with_personal(offering.luck_value())
        } else {
            LoadoutLuckRecord::with_global(offering.luck_value())
        }
    }
}

impl Offering {
    const fn luck_value(&self) -> Luck {
        match self {
            Offering::ChalkPouch | Offering::SaltPouch => k::SLIGHT_LUCK,
            Offering::CreamPouch | Offering::SaltStatuette => k::MODERATE_LUCK,
            Offering::IvoryPouch | Offering::SaltyLips => k::GREAT_LUCK,
        }
    }
    const fn luck_is_personal(&self) -> bool {
        matches!(
            self,
            Offering::ChalkPouch | Offering::CreamPouch | Offering::IvoryPouch
        )
    }

    pub fn iterator() -> OfferingIter {
        Self::iter()
    }
}

impl std::fmt::Display for Offering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Offering::ChalkPouch => "chalk pouch",
                Offering::CreamPouch => "cream pouch",
                Offering::IvoryPouch => "ivory pouch",
                Offering::SaltPouch => "salt pouch",
                Offering::SaltStatuette => "salt statuette",
                Offering::SaltyLips => "salty lips",
            }
        )
    }
}
