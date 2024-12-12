use super::loadout::Loadout;
use super::luck_record::{LoadoutPlayerConverter, PlayerLuckRecord};

#[derive(Debug, Clone, Copy)]
pub struct Player {
    loadout: Loadout,
    is_alive: bool,
}

impl Player {
    fn make_record_converter(&self) -> LoadoutPlayerConverter {
        LoadoutPlayerConverter::new(self.is_alive)
    }

    fn make_player_luck(&self) -> PlayerLuckRecord {
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
