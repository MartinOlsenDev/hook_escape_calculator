use super::luck::LuckScope;

pub enum Offering {
    ChalkPouch,
    CreamPouch,
    SaltPouch,
    SaltStatuette,
    IvoryPouch,
    SaltyLips,
}

impl Offering {
    pub fn luck_value(&self) -> f64 {
        match self {
            Offering::ChalkPouch | Offering::SaltPouch => 0.01,
            Offering::CreamPouch | Offering::SaltStatuette => 0.02,
            Offering::IvoryPouch | Offering::SaltyLips => 0.03,
        }
    }

    pub fn luck_scope(&self) -> LuckScope {
        match self {
            Offering::ChalkPouch | Offering::CreamPouch | Offering::IvoryPouch => LuckScope::Personal,
            Offering::SaltPouch | Offering::SaltStatuette | Offering::SaltyLips => LuckScope::Global
        }
    }
}