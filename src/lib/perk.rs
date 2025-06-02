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
#[cfg_attr(test, derive(EnumIter))]
pub enum PerkName {
    SlipperyMeat,
    UpTheAnte,
}

impl From<&Perk> for LoadoutLuckRecord {
    fn from(perk: &Perk) -> Self {
        match perk.name {
            PerkName::UpTheAnte => LoadoutLuckRecord::with_uta(uta_tier_percent(perk.tier)),
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
    let unhook_chance_record = LoadoutLuckRecord::with_personal(match tier {
        Tier::One => k::SM_TIER1,
        Tier::Two => k::SM_TIER2,
        Tier::Three => k::SM_TIER3,
    });
    let unhook_count_record = LoadoutLuckRecord::with_unhook_mod(3);
    &unhook_chance_record + &unhook_count_record
}

/// Module for generating arbitrary test values
#[cfg(test)]
pub mod arb {
    use super::*;
    use proptest::prelude::*;

    pub fn name() -> impl Strategy<Value = PerkName> {
        let perks: Vec<PerkName> = PerkName::iter().collect();
        prop::sample::select(perks)
    }
}
/// Actual test module
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn perk_count_actual_and_expected_match() {
        let perks: usize = PerkName::iter().count();
        let expected = super::super::constants::misc::COUNT_LUCK_PERKS;
        assert_eq!(perks, expected)
    }
}
