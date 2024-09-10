use crate::lib::backend::luck::CalculatedLuck;
use super::loadout::Loadout;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub loadout: Loadout,
    pub is_alive: bool,
}

// Perhaps the Up the Ante logic could also be in here, for simplification?

impl Default for Player {
    fn default() -> Self {
        Player {
            loadout: Loadout::default(),
            is_alive: true,
        }
    }
}
