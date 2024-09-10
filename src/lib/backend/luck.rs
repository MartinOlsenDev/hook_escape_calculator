use super::perk::UpTheAnte;
use super::player::Player;
use super::team::Team;

pub type PersonalLuck = f64;
pub type GlobalLuck = f64;

/// If the luck value is known at comp-time
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LuckSource {
    Calculated(CalculatedLuck),
    Dynamic(DynamicLuck),
}

impl LuckSource {
    pub fn is_dynamic(&self) -> bool {
        match self {
            LuckSource::Calculated(_) => false,
            LuckSource::Dynamic(_) => true,
        }
    }

    pub fn get_dynamic(&self) -> Option<DynamicLuck> {
        match self {
            LuckSource::Calculated(_) => None,
            LuckSource::Dynamic(d) => Some(*d),
        }
    }
    pub fn get_calculated(&self) -> Option<CalculatedLuck> {
        match self {
            LuckSource::Calculated(c) => Some(*c),
            LuckSource::Dynamic(_) => None,
        }
    }
}

/// Who the luck is known to affect and
/// how much.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalculatedLuck {
    Personal(PersonalLuck),
    Global(GlobalLuck),
}

impl CalculatedLuck {
    pub fn get_personal(&self) -> PersonalLuck {
        match self {
            CalculatedLuck::Personal(luck) => *luck,
            _ => 0.0,
        }
    }
    pub fn get_global(&self) -> GlobalLuck {
        match self {
            CalculatedLuck::Global(luck) => *luck,
            _ => 0.0,
        }
    }
}

/// When the luck value can be calculated,
/// if dynamic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicLuck {
    Team(TeamDynamicLuck),
}

/// List of all sources of Dynamic Luck within a team.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeamDynamicLuck {
    UpTheAnte(UpTheAnte),
}

impl TeamDynamicLuck {
    pub fn make_global_luck(&self, team: &Team, player: &Player) -> GlobalLuck {
        let living_count = team.alive_not_counting(player);
        match self {
            TeamDynamicLuck::UpTheAnte(perk) => {
                CalculatedLuck::Global(perk.make_luck(&living_count)).get_global()
            }
        }
    }
}

pub trait CalculatableLuck {
    fn personal_luck(&self) -> PersonalLuck;
    fn global_luck(&self) -> GlobalLuck;
}

#[cfg(test)]
mod tests {
    use super::super::offering::Offering;
    use super::super::perk::*;
    use super::*;

    #[test]
    fn personal_get() {
        let input = CalculatedLuck::Personal(0.01);
        let result = input.get_personal();
        assert_eq!(0.01, result);
    }
    #[test]
    fn personal_get_on_global() {
        let input = CalculatedLuck::Global(0.01);
        let result = input.get_personal();
        assert_eq!(0.0, result);
    }
    #[test]
    fn global_get() {
        let input = CalculatedLuck::Global(0.01);
        let result = input.get_global();
        assert_eq!(0.01, result);
    }
    #[test]
    fn slippery_to_luck() {
        let input = Perk::SlipperyMeat(SlipperyMeat::Two);
        let result = LuckSource::from(input);
        assert_eq!(
            Some(CalculatedLuck::Personal(0.03)),
            result.get_calculated()
        );
        assert_eq!(None, result.get_dynamic());
    }
    #[test]
    fn offering_to_luck() {
        let input = Offering::ChalkPouch;
        let result: LuckSource = input.into();
        assert_eq!(
            Some(CalculatedLuck::Personal(0.01)),
            result.get_calculated()
        );
        assert_eq!(None, result.get_dynamic());
    }

    #[test]
    fn up_the_ante() {
        let input = Perk::UpTheAnte(UpTheAnte::Two);
        let result: LuckSource = input.into();
        let expected = LuckSource::Dynamic(DynamicLuck::Team(TeamDynamicLuck::UpTheAnte(
            UpTheAnte::Two,
        )));
        assert_eq!(expected, result);
    }
}
