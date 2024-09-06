use super::living_count::LivingCount;
use super::luck::{
    CalculatableLuck, CalculatedLuck, DynamicLuck, Luck, LuckSource, TeamDynamicLuck,
};

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

impl From<Perk> for LuckSource {
    fn from(value: Perk) -> Self {
        match value {
            Perk::UpTheAnte(perk) => perk.into(),
            Perk::SlipperyMeat(perk) => perk.into(),
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
    pub fn make_luck(&self, living_count: &LivingCount) -> Luck {
        living_count.0 as Luck * self.get_multiplier()
    }
    fn get_multiplier(&self) -> Luck {
        match &self {
            Self::One => k::UTA_TIER1,
            Self::Two => k::UTA_TIER2,
            Self::Three => k::UTA_TIER3,
        }
    }
}

impl From<UpTheAnte> for LuckSource {
    fn from(value: UpTheAnte) -> Self {
        LuckSource::Dynamic(DynamicLuck::Team(TeamDynamicLuck::UpTheAnte(value)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlipperyMeat {
    One,
    Two,
    Three,
}

impl From<SlipperyMeat> for LuckSource {
    fn from(value: SlipperyMeat) -> Self {
        LuckSource::Calculated(value.personal_luck())
    }
}

impl CalculatableLuck for SlipperyMeat {
    fn personal_luck(&self) -> CalculatedLuck {
        CalculatedLuck::Personal(match &self {
            Self::One => k::SM_TIER1,
            Self::Two => k::SM_TIER2,
            Self::Three => k::SM_TIER3,
        })
    }
    fn global_luck(&self) -> CalculatedLuck {
        CalculatedLuck::Global(0.0)
    }
}
