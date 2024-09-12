use super::luck::{CalculatableLuck, GlobalLuck, PersonalLuck};
use crate::lib::backend::luck::{CalculatedLuck, LuckSource};

#[derive(Debug, Clone, Copy)]
pub enum Offering {
    ChalkPouch,
    CreamPouch,
    SaltPouch,
    SaltStatuette,
    IvoryPouch,
    SaltyLips,
}

impl From<Offering> for LuckSource {
    fn from(value: Offering) -> Self {
        use Offering::*;
        match value {
            ChalkPouch | CreamPouch | IvoryPouch => {
                LuckSource::Calculated(CalculatedLuck::Personal(value.personal_luck()))
            }
            SaltPouch | SaltStatuette | SaltyLips => LuckSource::Calculated(CalculatedLuck::Global(value.global_luck())),
        }
    }
}

impl CalculatableLuck for Offering {
    fn personal_luck(&self) -> PersonalLuck {
        match self {
            Offering::ChalkPouch => 0.01,
            Offering::CreamPouch => 0.02,
            Offering::IvoryPouch => 0.03,
            _ => 0.0,
        }
    }

    fn global_luck(&self) -> GlobalLuck {
        match self {
            Offering::SaltPouch => 0.01,
            Offering::SaltStatuette => 0.02,
            Offering::SaltyLips => 0.03,
            _ => 0.0,
        }
    }
}
