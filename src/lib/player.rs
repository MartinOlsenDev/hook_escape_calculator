use super::loadout::Loadout;
use super::luck_record::{LoadoutPlayerConverter, PlayerLuckRecord};
use super::offering::Offering;
use super::perk::{PerkName, Tier};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
    loadout: Loadout,
    is_alive: bool,
}

// Delegated Mutators
impl Player {
    pub fn set_offering(&mut self, offering: Option<Offering>) {
        self.loadout.set_offering(offering);
    }
    pub fn set_perk_tier(&mut self, name: PerkName, tier: Option<Tier>) {
        self.loadout.set_perk_tier(name, tier);
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
    pub fn get_perk_tier(&self, name: PerkName) -> Option<&Tier> {
        self.loadout.get_perk(name).map(|x| x.tier())
    }

    pub fn get_offering(&self) -> Option<&Offering> {
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
