use super::loadout::Loadout;
use super::luck_record::{LoadoutPlayerConverter, PlayerLuckRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
    loadout: Loadout,
    is_alive: bool,
}
// accessors
impl Player {
    pub fn get_mut_loadout(&mut self) -> &mut Loadout {
        &mut self.loadout
    }
    pub fn set_alive(&mut self) {
        self.is_alive = true;
    }
    pub fn set_dead(&mut self) {
        self.is_alive = false;
    }
}

impl Player {
    pub const fn is_alive(&self) -> bool {
        self.is_alive
    }
    fn make_record_converter(&self) -> LoadoutPlayerConverter {
        LoadoutPlayerConverter::new(self.is_alive)
    }

    pub fn make_player_luck(&self) -> PlayerLuckRecord {
        self.make_record_converter()
            .convert(self.loadout.collate_luck())
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            loadout: Loadout::default(),
            is_alive: true,
        }
    }
}
