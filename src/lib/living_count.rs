use super::constants::misc as k;

use nutype::nutype;

const MAX_CAP_U8: u8 = k::TEAM_MAX_CAPACITY as u8;

#[nutype(
    validate(less_or_equal = MAX_CAP_U8),
    derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, AsRef, Deref),
    default = 0
)]
pub struct LivingCount(u8);
