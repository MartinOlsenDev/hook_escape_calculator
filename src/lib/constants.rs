pub use {misc::*, offering::*, perk::*};

mod offering {
    pub const SLIGHT_LUCK: f64 = 0.01;
    pub const MODERATE_LUCK: f64 = 0.02;
    pub const GREAT_LUCK: f64 = 0.03;
}

mod perk {
    pub const UTA_TIER1: f64 = 0.01;
    pub const UTA_TIER2: f64 = 0.02;
    pub const UTA_TIER3: f64 = 0.03;

    pub const SM_TIER1: f64 = 0.02;
    pub const SM_TIER2: f64 = 0.03;
    pub const SM_TIER3: f64 = 0.04;
}

mod misc {
    pub const COUNT_ALL_KNOWN_LUCK_PERKS: usize = 2;

    pub const TEAM_MAX_CAPACITY: usize = 4;

    pub const BASE_UNHOOK_CHANCE: f64 = 0.04;
}
