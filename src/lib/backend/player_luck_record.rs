use frunk::monoid::Monoid;
use frunk::Semigroup;

use f64 as Luck;

/// A record that represents a players luck items such that
/// two personal lucks are summed rather than list appended
#[derive(PartialEq, Debug, Clone, Copy, Default)]
struct LoadoutLuckRecord {
    personal: Luck,
    global: Luck,
    up_the_ante_coeff: Option<Luck>,
    additional_unhooks: i8,
}

/// A semigroup instance for LoadoutLuckRecord. It is assumed that
/// the input player luck items does not contain multiple instances
/// of Up the Ante. If there are, the second is discarded.
impl Semigroup for LoadoutLuckRecord {
    fn combine(&self, other: &Self) -> Self {
        let &LoadoutLuckRecord {
            personal: personal0,
            global: global0,
            up_the_ante_coeff: uta0,
            additional_unhooks: au0,
        } = &self;
        let &LoadoutLuckRecord {
            personal: personal1,
            global: global1,
            up_the_ante_coeff: uta1,
            additional_unhooks: au1,
        } = other;
        LoadoutLuckRecord {
            personal: personal0 + personal1,
            global: global0 + global1,
            up_the_ante_coeff: uta0.or(uta1),
            additional_unhooks: au0 + au1,
        }
    }
}

impl Monoid for LoadoutLuckRecord {
    fn empty() -> Self {
        LoadoutLuckRecord {
            personal: Luck::default(),
            global: Luck::default(),
            up_the_ante_coeff: None,
            additional_unhooks: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LoadoutPlayerConverter {
    is_alive: bool,
}

impl LoadoutPlayerConverter {
    pub fn new(is_alive: bool) -> Self {
        Self { is_alive }
    }
    pub fn convert(&self, loadout: LoadoutLuckRecord) -> PlayerLuckRecord {
        let LoadoutLuckRecord {
            personal,
            global,
            up_the_ante_coeff,
            additional_unhooks,
        } = loadout;
        // This line is what causes dead players to not contribute their
        // Up the Ante to the global luck.
        let up_the_ante_coeff = up_the_ante_coeff.filter(|_| self.is_alive);

        PlayerLuckRecord {
            personal,
            global,
            up_the_ante_coeff,
            additional_unhooks,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PlayerLuckRecord {
    personal: Luck,
    global: Luck,
    up_the_ante_coeff: Option<Luck>,
    additional_unhooks: i8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PlayerTeamConverter {
    living_count: u8,
}

impl PlayerTeamConverter {
    pub fn new(living_count: u8) -> Self {
        Self { living_count }
    }
    pub fn convert(&self, plr: &PlayerLuckRecord) -> TeamLuckRecord {
        let &PlayerLuckRecord {
            personal,
            global,
            up_the_ante_coeff: uta_coeff,
            additional_unhooks,
        } = plr;

        let uta_contribution = uta_coeff
            .map(|x| x * f64::from(self.living_count))
            .unwrap_or(0.0);

        let final_global = global + uta_contribution;

        TeamLuckRecord {
            global: final_global,
            personals: vec![(personal, additional_unhooks)],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeamLuckRecord {
    global: Luck,
    personals: Vec<(Luck, i8)>,
}

///TODO: This is a good optimization oppurtunity
impl Semigroup for TeamLuckRecord {
    fn combine(&self, other: &Self) -> Self {
        let &TeamLuckRecord {
            global: global0,
            personals: personals0,
        } = &self;

        let TeamLuckRecord {
            global: global1,
            personals: personals1,
        } = other;

        let mut personals = personals0.clone();
        personals.extend(personals1.clone());

        TeamLuckRecord {
            global: global0 + global1,
            personals,
        }
    }
}

impl Monoid for TeamLuckRecord {
    fn empty() -> Self {
        TeamLuckRecord {
            global: 0.0,
            personals: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial_default_comparison() {
        let a = LoadoutLuckRecord::default();
        let b = LoadoutLuckRecord::default();
        assert_eq!(a, b);
    }
}
