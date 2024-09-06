use super::luck::CalculatableLuck;
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
                LuckSource::Calculated(value.personal_luck().clone())
            }
            SaltPouch | SaltStatuette | SaltyLips => LuckSource::Calculated(value.global_luck()),
        }
    }
}

impl CalculatableLuck for Offering {
    fn personal_luck(&self) -> CalculatedLuck {
        CalculatedLuck::Personal(match self {
            Offering::ChalkPouch => 0.01,
            Offering::CreamPouch => 0.02,
            Offering::IvoryPouch => 0.03,
            _ => 0.0,
        })
    }

    fn global_luck(&self) -> CalculatedLuck {
        CalculatedLuck::Global(match self {
            Offering::SaltPouch => 0.01,
            Offering::SaltStatuette => 0.02,
            Offering::SaltyLips => 0.03,
            _ => 0.0,
        })
    }
}
