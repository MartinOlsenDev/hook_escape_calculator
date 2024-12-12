use super::luck_producer::LuckProducer;
use super::luck_record::LoadoutLuckRecord;
use super::luck_record::Luck;
use frunk::Semigroup;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Perk {
    UpTheAnte(UpTheAnte),
    SlipperyMeat(SlipperyMeat),
}

impl From<Perk> for LoadoutLuckRecord {
    fn from(value: Perk) -> Self {
        match value {
            Perk::UpTheAnte(perk) => (&perk).into(),
            Perk::SlipperyMeat(perk) => (&perk).into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpTheAnte {
    One,
    Two,
    Three,
}

impl UpTheAnte {
    fn get_multiplier(&self) -> f64 {
        match &self {
            Self::One => k::UTA_TIER1,
            Self::Two => k::UTA_TIER2,
            Self::Three => k::UTA_TIER3,
        }
    }
}
impl From<&UpTheAnte> for LoadoutLuckRecord {
    fn from(item: &UpTheAnte) -> Self {
        Self::from_uta(item.get_multiplier())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlipperyMeat {
    One,
    Two,
    Three,
}

impl SlipperyMeat {
    fn get_luck_value(&self) -> f64 {
        match self {
            SlipperyMeat::One => k::SM_TIER1,
            SlipperyMeat::Two => k::SM_TIER2,
            SlipperyMeat::Three => k::SM_TIER3,
        }
    }
}

impl From<&SlipperyMeat> for LoadoutLuckRecord {
    fn from(item: &SlipperyMeat) -> Self {
        let unhook_chance_mod = Self::from_personal(item.get_luck_value());
        let unhook_count_mod = Self::from_unhook_mod(3);
        unhook_chance_mod.combine(&unhook_count_mod)
    }
}
