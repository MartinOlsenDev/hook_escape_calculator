use std::error;

pub struct LivingCount(pub u8);

impl From<LivingCount> for u8 {
    fn from(value: LivingCount) -> Self {
        value.0
    }
}

// todo: Create proper error type
impl TryFrom<u8> for LivingCount {
    type Error = Box<dyn error::Error>;
    fn try_from(value: u8) -> Result<Self, Box<dyn error::Error>> {
        if value <= 4 {
            Ok(Self(value))
        } else {
            Err("invalid living count".into())
        }
    }
}
