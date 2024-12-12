use super::loadout::Loadout;
use super::luck_record::{LoadoutLuckRecord, LoadoutPlayerConverter, Luck, PlayerLuckRecord};
//use super::team::Team;
//use crate::lib::backend::living_count::LivingCount;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub loadout: Loadout,
    pub is_alive: bool,
}

impl Player {}

impl Default for Player {
    fn default() -> Self {
        Player {
            loadout: Loadout::default(),
            is_alive: true,
        }
    }
}
