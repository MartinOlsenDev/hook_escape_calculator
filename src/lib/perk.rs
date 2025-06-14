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

impl Perk {
    pub const fn new(name: PerkName, tier: Tier) -> Perk {
        Perk { name, tier }
    }
    pub const fn set_tier(&mut self, tier: Tier) {
        self.tier = tier
    }
}

#[nutype(
    derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, AsRef, Deref),
    default = None
)]
pub struct PerkSlot(Option<Perk>);

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
    use prop::collection::HashSetStrategy;
    use proptest::prelude::*;

    pub fn name() -> impl Strategy<Value = PerkName> {
        let perks: Vec<PerkName> = PerkName::iter().collect();
        prop::sample::select(perks)
    }
    pub fn dinstinct_names(n: usize) -> HashSetStrategy<impl Strategy<Value = PerkName>> {
        proptest::collection::hash_set(name(), n)
    }
    pub fn tier() -> impl Strategy<Value = Tier> {
        let tiers: Vec<_> = Tier::iter().collect();
        prop::sample::select(tiers)
    }
    prop_compose! {
        fn some_tier_slot()(tier in tier()) -> TierSlot {
            TierSlot::new(Some(tier))
        }
    }
    pub fn tier_slot() -> BoxedStrategy<TierSlot> {
        prop_oneof![Just(TierSlot::new(None)), some_tier_slot()].boxed()
    }
    prop_compose! {
        pub fn perk()(name in name(), tier in tier()) -> Perk {
            Perk::new(name, tier)
        }
    }
    prop_compose! {
        fn some_perk_slot()(perk in perk()) -> PerkSlot {
            PerkSlot::new(Some(perk))
        }
    }
    pub fn perk_slot() -> BoxedStrategy<PerkSlot> {
        prop_oneof![Just(PerkSlot::new(None)), some_perk_slot()].boxed()
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
