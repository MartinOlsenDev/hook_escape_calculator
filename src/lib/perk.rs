use derive_getters::Getters;
use nutype::nutype;
use strum::{EnumIter, IntoEnumIterator};

use crate::constants::perk_luck as k;

use super::luck_record::LoadoutLuckRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Getters)]
pub struct Perk {
    name: PerkName,
    tier: Tier,
}

#[nutype(
    derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, AsRef, Deref),
    default = None
)]
pub struct PerkSlot(Option<Perk>);

impl Perk {
    pub const fn new(name: PerkName, tier: Tier) -> Perk {
        Perk { name, tier }
    }
    pub const fn get_tier_mut(&mut self) -> &mut Tier {
        &mut self.tier
    }
    pub const fn set_tier(&mut self, tier: Tier) {
        self.tier = tier
    }
}

#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, Hash))]
pub struct TierSlot(Option<Tier>);

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
        match perk.name {
            PerkName::UpTheAnte => LoadoutLuckRecord::from_uta(uta_tier_percent(perk.tier)),
            PerkName::SlipperyMeat => slippery_meat_record(perk.tier),
        }
    }
}

const fn uta_tier_percent(tier: Tier) -> f64 {
    match tier {
        Tier::One => k::UTA_TIER1,
        Tier::Two => k::UTA_TIER2,
        Tier::Three => k::UTA_TIER3,
    }
}

fn slippery_meat_record(tier: Tier) -> LoadoutLuckRecord {
    let unhook_chance_record = LoadoutLuckRecord::from_personal(match tier {
        Tier::One => k::SM_TIER1,
        Tier::Two => k::SM_TIER2,
        Tier::Three => k::SM_TIER3,
    });
    let unhook_count_record = LoadoutLuckRecord::from_unhook_mod(3);
    &unhook_chance_record + &unhook_count_record
}
