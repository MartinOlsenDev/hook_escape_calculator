mod constants {
    pub const UTA_TIER1: f64 = 0.01;
    pub const UTA_TIER2: f64 = 0.02;
    pub const UTA_TIER3: f64 = 0.03;

    pub const SM_TIER1: f64 = 0.02;
    pub const SM_TIER2: f64 = 0.03;
    pub const SM_TIER3: f64 = 0.04;

}
use constants as k;

pub enum UpTheAnte {
    One,
    Two,
    Three
}

impl UpTheAnte {
    pub fn luck_mod(&self) -> f64 {
        match &self {
            Self::One => k::UTA_TIER1,
            Self::Two => k::UTA_TIER2,
            Self::Three => k::UTA_TIER3
        }
    }
}

pub enum SlipperyMeat {
    One,
    Two,
    Three
}

impl SlipperyMeat {
    pub fn luck_mod(&self) -> f64 {
        match &self {
            Self::One => k::SM_TIER1,
            Self::Two => k::SM_TIER2,
            Self::Three => k::SM_TIER3
        }
    }
}