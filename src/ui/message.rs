use hook_escape_calculator::{offering, perk, update};
use iced::window;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Message {
    UpdateSurvivor(update::SurvivorUpdate),
    OpenHelp,
    CloseHelp,
    ExitApp,
    StartApp,
    CloseWindow(window::Id),
    Noop,
}

impl Message {
    pub fn new_surv_update(id: usize, update: update::SurvivorUpdateData) -> Message {
        Message::UpdateSurvivor(SurvivorUpdate { id, update })
    }
}
