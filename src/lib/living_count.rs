pub struct LivingCount(pub u8);

impl From<LivingCount> for u8 {
    fn from(value: LivingCount) -> Self {
        value.0
    }
}

impl TryFrom<u8> for LivingCount {
    type Error = (); // only one possible cause of failure: vaule >4
    fn try_from(value: u8) -> Result<Self, ()> {
        if value <= 4 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}
