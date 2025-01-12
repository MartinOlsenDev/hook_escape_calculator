use super::luck_record::LoadoutLuckRecord;
use super::luck_record::Luck;
use frunk::Semigroup;

pub const COUNT_ALL_KNOWN_LUCK_PERKS: usize = 2;

mod constants {
    use super::Luck;
    pub const UTA_TIER1: Luck = 0.01;
    pub const UTA_TIER2: Luck = 0.02;
    pub const UTA_TIER3: Luck = 0.03;

    pub const SM_TIER1: Luck = 0.02;
    pub const SM_TIER2: Luck = 0.03;
    pub const SM_TIER3: Luck = 0.04;
}
use constants as k;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Perk {
    name: PerkName,
    tier: Tier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tier {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PerkName {
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
