use konst;

pub mod offering_luck {
    pub const SLIGHT_LUCK: f64 = 0.01;
    pub const MODERATE_LUCK: f64 = 0.02;
    pub const GREAT_LUCK: f64 = 0.03;
}

pub mod perk_luck {
    pub const UTA_TIER1: f64 = 0.01;
    pub const UTA_TIER2: f64 = 0.02;
    pub const UTA_TIER3: f64 = 0.03;

    pub const SM_TIER1: f64 = 0.02;
    pub const SM_TIER2: f64 = 0.03;
    pub const SM_TIER3: f64 = 0.04;
}

pub mod misc {
    use super::*;

    pub const COUNT_LUCK_PERKS: usize = 2;
    pub const MAX_PERKS: usize = 4;
    pub const PERKSLOT_COUNT: usize = konst::min!(MAX_PERKS, COUNT_LUCK_PERKS);

    pub const TEAM_MAX_CAPACITY: usize = 4;

    pub const BASE_UNHOOK_CHANCE: f64 = 0.04;
}
