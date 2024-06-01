pub enum UpTheAnte {
    One,
    Two,
    Three
}

impl UpTheAnte {
    pub fn luck_mod(&self) -> f64 {
        match &self {
            Self::One => 0.01,
            Self::Two => 0.02,
            Self::Three => 0.03
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
            Self::One => 0.02,
            Self::Two => 0.03,
            Self::Three => 0.04
        }
    }
}