pub enum LivingCount {
    Zero,
    One,
    Two,
    Three,
    Four
}

impl From<LivingCount> for u8 {
    fn from(value: LivingCount) -> Self {
        match value {
            LivingCount::Zero => 0_u8,
            LivingCount::One => 1_u8,
            LivingCount::Two => 2_u8,
            LivingCount::Three => 3_u8,
            LivingCount::Four => 4_u8
        }
    }
}

impl TryFrom<u8> for LivingCount {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LivingCount::Zero),
            1 => Ok(LivingCount::One),
            2 => Ok(LivingCount::Two),
            3 => Ok(LivingCount::Three),
            4 => Ok(LivingCount::Four),
            _ => Err(()),
        }
    }
}