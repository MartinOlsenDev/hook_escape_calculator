use super::player::Player;
use super::perks::UpTheAnte;
use super::team::Team;

pub type Luck = f64;

/// If the luck value is known at comp-time
#[derive(Debug, Clone, Copy)]
pub enum LuckSource {
    Calculated(CalculatedLuck),
    Dynamic(DynamicLuck)
}

impl LuckSource {
    pub fn is_dynamic(&self) -> bool {
        match self {
            LuckSource::Calculated(_) => false,
            LuckSource::Dynamic(_) => true
        }
    }

    pub fn get_dynamic(&self) -> Option<DynamicLuck> {
        match self {
            LuckSource::Calculated(_) => None,
            LuckSource::Dynamic(d) => Some(*d)
        }
    }
}

/// Who the luck is known to affect and
/// how much.
#[derive(Debug, Clone, Copy)]
pub enum CalculatedLuck {
    Personal(Luck),
    Global(Luck)
}

impl CalculatedLuck {
    pub fn get_personal(&self) -> Luck {
        match self {
            CalculatedLuck::Personal(luck) => *luck,
            _ => 0.0
        }
    }
    pub fn get_global(&self) -> Luck {
        match self {
            CalculatedLuck::Global(luck) => *luck,
            _ => 0.0
        }
    }
}

/// When the luck value can be calculated,
/// if dynamic
#[derive(Debug, Clone, Copy)]
pub enum DynamicLuck {
    Team(TeamDynamicLuck)
}

/// List of all sources of Dynamic Luck within a team.
#[derive(Debug, Clone, Copy)]
pub enum TeamDynamicLuck {
    UpTheAnte(UpTheAnte)
}

impl TeamDynamicLuck {
    pub fn make_global_luck(&self, team: &Team, player: &Player) -> CalculatedLuck {
        let living_count = team.alive_not_counting(player);
        match self {
            TeamDynamicLuck::UpTheAnte(perk) => CalculatedLuck::Global(perk.make_luck(&living_count))
        }
    }
}

pub trait CalculatableLuck {
    fn personal_luck(&self) -> CalculatedLuck;
    fn global_luck(&self) -> CalculatedLuck;
}