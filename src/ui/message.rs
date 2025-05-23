use hook_escape_calculator::{offering, perk};
use iced::window;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Message {
    UpdateSurvivor(SurvivorUpdate),
    OpenHelp,
    CloseHelp,
    ExitApp,
    StartApp,
    CloseWindow(window::Id),
    Noop,
}

impl Message {
    pub fn new_surv_update(id: usize, update: SurvivorUpdateData) -> Message {
        Message::UpdateSurvivor(SurvivorUpdate { id, update })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SurvivorUpdate {
    pub id: usize,
    pub update: SurvivorUpdateData,
}

impl SurvivorUpdate {
    pub fn slippery(id: usize, tier: Option<perk::Tier>) -> SurvivorUpdate {
        SurvivorUpdate {
            id,
            update: SurvivorUpdateData::Perk(PerkUpdate {
                perk: perk::PerkName::SlipperyMeat,
                value: tier,
            }),
        }
    }
    pub fn uta(id: usize, tier: Option<perk::Tier>) -> SurvivorUpdate {
        SurvivorUpdate {
            id,
            update: SurvivorUpdateData::Perk(PerkUpdate {
                perk: perk::PerkName::UpTheAnte,
                value: tier,
            }),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SurvivorUpdateData {
    Perk(PerkUpdate),
    Offering(Option<offering::Offering>),
    Life(bool),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PerkUpdate {
    pub perk: perk::PerkName,
    pub value: Option<perk::Tier>,
}
