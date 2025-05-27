use super::{
    loadout::Loadout,
    luck_record::{LoadoutPlayerConverter, PlayerLuckRecord},
    offering::OfferingSlot,
    perk::{PerkName, Tier},
    update::SurvivorUpdateData as SUD,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
    loadout: Loadout,
    is_alive: bool,
}

impl Player {
    pub fn alter(&mut self, update: SUD) {
        match update {
            SUD::Life(x) => self.is_alive = x,
            SUD::LoadoutUpdate(x) => self.loadout.alter(x),
        };
    }
}

// Delegated Getters
impl Player {
    pub fn get_perk_tier(&self, name: PerkName) -> Option<&Tier> {
        self.loadout
            .get_perk(name)
            .as_ref()
            .as_ref()
            .map(|perk| perk.tier())
    }
    pub fn get_offering(&self) -> &OfferingSlot {
        self.loadout.get_offering()
    }
}

// Getters
impl Player {
    pub const fn is_alive(self) -> bool {
        self.is_alive
    }
    pub const fn is_dead(self) -> bool {
        !self.is_alive()
    }
    fn make_record_converter(self) -> LoadoutPlayerConverter {
        LoadoutPlayerConverter::new(self.is_alive)
    }

    // Consider placing in sub-module
    pub fn make_player_luck(self) -> PlayerLuckRecord {
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
