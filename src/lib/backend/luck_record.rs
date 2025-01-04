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
        LoadoutLuckRecord {
            personal: self.personal + other.personal,
            global: self.global + other.global,
            up_the_ante_coeff: self.up_the_ante_coeff.or(other.up_the_ante_coeff),
            additional_unhooks: self.additional_unhooks + other.additional_unhooks,
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
            mut up_the_ante_coeff,
            additional_unhooks,
        } = loadout;
        // This line is what causes dead players to not contribute their
        // Up the Ante to the global luck.
        up_the_ante_coeff = up_the_ante_coeff.filter(|_| self.is_alive);

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
        let LoadoutLuckRecord {
            personal,
            global,
            up_the_ante_coeff,
            additional_unhooks,
        } = plr.0;

        let uta_contribution = up_the_ante_coeff
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

impl TeamLuckRecord {
    pub fn from_global(luck: Luck) -> Self {
        TeamLuckRecord {
            global: luck,
            personals: ArrayVec::new(),
        }
    }
    pub fn luck_unhook_mod_pairs_iter<'a>(&'a self) -> impl Iterator<Item = (Luck, i8)> + 'a {
        self.personals
            .iter()
            .map(|(luck, unhook_mod)| (luck + self.global, *unhook_mod))
    }
    pub fn make_single_and_total_unhook_pairs<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Luck, Luck)> + 'a {
        self.luck_unhook_mod_pairs_iter()
            .map(|(luck, unhook_count)| {
                let chance_fail: Luck = 1.0 - luck;
                let chance_fail_all = chance_fail.powi(i32::from(unhook_count) + 3);
                let chance_succeed_once = 1.0 - chance_fail_all;
                (luck, chance_succeed_once)
            })
    }
    //TODO: This should probably be done at callsite by a front-end
    // that's received the pairs
    pub fn make_single_and_total_unhook_strings(
        self,
    ) -> ArrayVec<(String, String), TEAM_MAX_CAPACITY> {
        self.make_single_and_total_unhook_pairs()
            .map(|(single, all)| (format!("{0:.2}", single), format!("{0:.2}", all)))
            .collect()
    }
}

///TODO: This is a good optimization oppurtunity
impl Semigroup for TeamLuckRecord {
    fn combine(&self, other: &Self) -> Self {
        let personals = {
            let mut personals = self.personals.clone();
            personals.extend(other.personals.clone());
            personals
        };

        TeamLuckRecord {
            global: self.global + other.global,
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
