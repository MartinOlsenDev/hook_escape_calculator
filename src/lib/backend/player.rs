use super::loadout::Loadout;
use super::team::Team;
use crate::lib::backend::living_count::LivingCount;
use crate::lib::backend::luck::{DynamicLuck, GlobalLuck, PersonalLuck};

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub loadout: Loadout,
    pub is_alive: bool,
}

// Perhaps the Up the Ante logic could also be in here, for simplification?

impl Player {
    pub fn make_max_unhook(&self) -> u8 {
        self.loadout.make_max_unhook()
    }

    pub fn make_personal_luck(&self) -> PersonalLuck {
        self.loadout.make_personal_luck()
    }

    pub fn make_global_luck(&self) -> GlobalLuck {
        self.loadout.make_global_luck()
    }

    fn make_dyn_luck(&self) -> Vec<DynamicLuck> {
        self.loadout.get_dyn_luck()
    }

    pub fn make_ante_luck(&self, team: &Team) -> GlobalLuck {
        if !self.is_alive {
            return 0.0;
        }

        self.loadout
            .get_team_uta()
            .map(|perk| perk.make_global_luck(team, &self))
            .unwrap_or(0.0)
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
