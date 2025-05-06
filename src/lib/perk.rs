use super::luck_record::LoadoutLuckRecord;
use derive_getters::Getters;
use frunk::Semigroup;
use strum::{EnumIter, IntoEnumIterator};

use crate::constants as k;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Getters)]
pub struct Perk {
    name: PerkName,
    tier: Tier,
}
pub type PerkSlot = Option<Perk>;

impl Perk {
    pub fn new(name: PerkName, tier: Tier) -> Perk {
        Perk { name, tier }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Tier {
    One,
    Two,
    Three,
}

impl Tier {
    pub fn iterator() -> TierIter {
        Self::iter()
    }
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tier::One => "one",
                Tier::Two => "two",
                Tier::Three => "three",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PerkName {
    SlipperyMeat,
    UpTheAnte,
}

impl From<&Perk> for LoadoutLuckRecord {
    fn from(perk: &Perk) -> Self {
        match (perk.name, perk.tier) {
            (PerkName::UpTheAnte, Tier::One) => LoadoutLuckRecord::from_uta(k::UTA_TIER1),
            (PerkName::UpTheAnte, Tier::Two) => LoadoutLuckRecord::from_uta(k::UTA_TIER2),
            (PerkName::UpTheAnte, Tier::Three) => LoadoutLuckRecord::from_uta(k::UTA_TIER3),
            (PerkName::SlipperyMeat, tier) => slippery_meat_record(tier),
        }
    }
}

fn slippery_meat_record(tier: Tier) -> LoadoutLuckRecord {
    let unhook_chance_record = LoadoutLuckRecord::from_personal(match tier {
        Tier::One => k::SM_TIER1,
        Tier::Two => k::SM_TIER2,
        Tier::Three => k::SM_TIER3,
    });
    let unhook_count_record = LoadoutLuckRecord::from_unhook_mod(3);
    unhook_chance_record.combine(&unhook_count_record)
}
