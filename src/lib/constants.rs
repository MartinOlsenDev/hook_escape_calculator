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

/// Observations are things we know about the universe of the application that the
/// compiler cannot prove. We can test against our observations of the world
/// in tests.
#[cfg(test)]
pub mod observations {
    // awful, awful pow implementation--but does work in const
    // context for small powers
    const fn const_power(x: f64, p: i8) -> f64 {
        if p <= 0 {
            1.
        } else {
            x * const_power(x, p - 1)
        }
    }

    pub const MIN_UNHOOK_ATTEMPTS: i8 = 3;
    pub const MAX_UNHOOK_ATTEMPTS: i8 = 6;

    pub const MAX_SINGLE_LUCK: f64 = 0.56;
    pub const MIN_SINGLE_LUCK: f64 = 0.04;
    pub const MIN_MULTIPLE_LUCK: f64 = 1. - const_power(1. - MIN_SINGLE_LUCK, MIN_UNHOOK_ATTEMPTS);
    pub const MAX_MULTIPLE_LUCK: f64 = 1. - const_power(1. - MAX_SINGLE_LUCK, MAX_UNHOOK_ATTEMPTS);
}
