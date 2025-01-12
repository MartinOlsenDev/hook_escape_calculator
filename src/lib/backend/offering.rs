use super::luck_record::{LoadoutLuckRecord, Luck};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Offering {
    ChalkPouch,
    CreamPouch,
    SaltPouch,
    SaltStatuette,
    IvoryPouch,
    SaltyLips,
}

impl From<&Offering> for LoadoutLuckRecord {
    fn from(offering: &Offering) -> Self {
        match offering.luck_is_personal() {
            true => LoadoutLuckRecord::from_personal(offering.luck_value()),
            false => LoadoutLuckRecord::from_global(offering.luck_value()),
        }
    }
}

impl Offering {
    const fn luck_value(&self) -> Luck {
        match self {
            Offering::ChalkPouch | Offering::SaltPouch => 0.01,
            Offering::CreamPouch | Offering::SaltStatuette => 0.02,
            Offering::IvoryPouch | Offering::SaltyLips => 0.03,
        }
    }
    const fn luck_is_personal(&self) -> bool {
        match self {
            Offering::ChalkPouch | Offering::CreamPouch | Offering::IvoryPouch => true,
            _ => false,
        }
    }
}
