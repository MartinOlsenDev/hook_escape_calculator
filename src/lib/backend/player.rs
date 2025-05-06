use derive_getters::Getters;

use super::loadout::Loadout;
use super::luck_record::{LoadoutPlayerConverter, PlayerLuckRecord};
use super::offering::Offering;
use super::perk::Tier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
    loadout: Loadout,
    is_alive: bool,
}

// Delegated Mutators
impl Player {
    pub fn set_slippery(&mut self, tier: Option<Tier>) {
        self.loadout.set_slippery(tier);
    }
    pub fn set_uta(&mut self, tier: Option<Tier>) {
        self.loadout.set_uta(tier);
    }
    pub fn set_offering(&mut self, offering: Option<Offering>) {
        self.loadout.set_offering(offering);
    }
}

// Mutators
impl Player {
    pub fn set_alive(&mut self) {
        self.is_alive = true;
    }
    pub fn set_dead(&mut self) {
        self.is_alive = false;
    }
}

// Delegated Getters
impl Player {
    pub fn get_slippery_tier(&self) -> Option<crate::perk::Tier> {
        self.loadout.get_slippery().map(|x| x.tier().clone())
    }
    pub fn get_uta_tier(&self) -> Option<crate::perk::Tier> {
        self.loadout.get_uta().map(|x| x.tier().clone())
    }
    pub fn get_offering(&self) -> Option<crate::offering::Offering> {
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
