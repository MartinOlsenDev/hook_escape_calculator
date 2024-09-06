use super::loadout::Loadout;

pub struct Player {
    pub loadout: Loadout,
    pub is_alive: bool
}

impl Default for Player {
    fn default() -> Self {
        todo!()
    }
}
