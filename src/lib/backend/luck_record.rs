use super::team::TEAM_MAX_CAPACITY;
use arrayvec::ArrayVec;
use frunk::monoid::Monoid;
use frunk::Semigroup;

pub type Luck = f64;

/// A record that represents a players luck items such that
/// two personal lucks are summed rather than list appended
#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub struct LoadoutLuckRecord {
    personal: Luck,
    global: Luck,
    up_the_ante_coeff: Option<Luck>,
    additional_unhooks: i8,
}

/// Init methods
impl LoadoutLuckRecord {
    pub const fn from_personal(personal: Luck) -> Self {
        Self {
            personal,
            global: 0.0,
            up_the_ante_coeff: None,
            additional_unhooks: 0,
        }
    }
    pub const fn from_global(global: Luck) -> Self {
        Self {
            personal: 0.0,
            global,
            up_the_ante_coeff: None,
            additional_unhooks: 0,
        }
    }
    pub const fn from_uta(uta: Luck) -> Self {
        Self {
            personal: 0.0,
            global: 0.0,
            up_the_ante_coeff: Some(uta),
            additional_unhooks: 0,
        }
    }
    pub const fn from_unhook_mod(additional_unhooks: i8) -> Self {
        Self {
            personal: 0.0,
            global: 0.0,
            up_the_ante_coeff: None,
            additional_unhooks,
        }
    }
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
pub struct LoadoutPlayerConverter {
    is_alive: bool,
}

impl LoadoutPlayerConverter {
    pub const fn new(is_alive: bool) -> Self {
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

        PlayerLuckRecord(LoadoutLuckRecord {
            personal,
            global,
            up_the_ante_coeff,
            additional_unhooks,
        })
    }
}

///TODO: Convert this into its own struct with named fields. Change
///from Option<f64> to f64 for up the ante. then make
///LoadoutPlayerConverter, as well as the associated caller
///function in player.rs
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerLuckRecord(pub LoadoutLuckRecord);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerTeamConverter {
    living_other_than_self_count: u8,
}

impl PlayerTeamConverter {
    pub const fn new(living_other_than_self_count: u8) -> Self {
        Self {
            living_other_than_self_count,
        }
    }
    pub fn convert(&self, plr: &PlayerLuckRecord) -> TeamLuckRecord {
        let &LoadoutLuckRecord {
            personal,
            global,
            up_the_ante_coeff: uta_coeff,
            additional_unhooks,
        } = &plr.0;

        let uta_contribution = uta_coeff
            .map(|x| x * f64::from(self.living_other_than_self_count))
            .unwrap_or(0.0);

        let final_global = global + uta_contribution;
        let personal_data = {
            let mut personal_data = ArrayVec::new();
            personal_data.push((personal, additional_unhooks));
            personal_data
        };

        TeamLuckRecord {
            global: final_global,
            personals: personal_data,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeamLuckRecord {
    global: Luck,
    personals: ArrayVec<(Luck, i8), TEAM_MAX_CAPACITY>,
}

///TODO: This is a good optimization oppurtunity
impl Semigroup for TeamLuckRecord {
    fn combine(&self, other: &Self) -> Self {
        let TeamLuckRecord {
            global: global0,
            personals: personals0,
        } = self;

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
            personals: ArrayVec::new(),
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
