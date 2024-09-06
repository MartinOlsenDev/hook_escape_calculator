use super::loadout::Loadout;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub loadout: Loadout,
    pub is_alive: bool
}

impl Default for Player {
    fn default() -> Self {
        Player {
            loadout: Loadout::default(),
            is_alive: true
        }
    }
}
