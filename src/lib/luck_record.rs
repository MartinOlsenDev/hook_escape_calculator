use arrayvec::ArrayVec;

use crate::constants::misc::TEAM_MAX_CAPACITY;

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

/// An add instance for `LoadoutLuckRecord`. It is assumed that
/// the input player luck items does not contain multiple instances
/// of Up the Ante. If there are, the second is discarded.
impl std::ops::Add for &LoadoutLuckRecord {
    type Output = LoadoutLuckRecord;

    fn add(self, other: Self) -> Self::Output {
        LoadoutLuckRecord {
            personal: self.personal + other.personal,
            global: self.global + other.global,
            up_the_ante_coeff: self.up_the_ante_coeff.or(other.up_the_ante_coeff),
            additional_unhooks: self.additional_unhooks + other.additional_unhooks,
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
    pub const fn convert(self, loadout: LoadoutLuckRecord) -> PlayerLuckRecord {
        let LoadoutLuckRecord {
            personal,
            global,
            mut up_the_ante_coeff,
            additional_unhooks,
        } = loadout;
        // This line is what causes dead players to not contribute their
        // Up the Ante to the global luck.
        up_the_ante_coeff = match self.is_alive {
            true => up_the_ante_coeff,
            false => None
        };

        PlayerLuckRecord(LoadoutLuckRecord {
            personal,
            global,
            up_the_ante_coeff,
            additional_unhooks,
        })
    }
}

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
    pub fn convert(self, plr: &PlayerLuckRecord) -> TeamLuckRecord {
        let LoadoutLuckRecord {
            personal,
            global,
            up_the_ante_coeff,
            additional_unhooks,
        } = plr.0;

        let uta_contribution =
            up_the_ante_coeff.map_or(0.0, |x| x * f64::from(self.living_other_than_self_count));

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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TeamLuckRecord {
    global: Luck,
    personals: ArrayVec<(Luck, i8), { TEAM_MAX_CAPACITY }>,
}

impl TeamLuckRecord {
    pub const fn from_global(luck: Luck) -> Self {
        TeamLuckRecord {
            global: luck,
            personals: ArrayVec::new_const(),
        }
    }
    pub fn luck_unhook_mod_pairs_iter(&self) -> impl Iterator<Item = (Luck, i8)> + '_ {
        self.personals
            .iter()
            .map(|(luck, unhook_mod)| (luck + self.global, *unhook_mod))
    }
    pub fn make_single_and_total_unhook_pairs(&self) -> impl Iterator<Item = (Luck, Luck)> + '_ {
        self.luck_unhook_mod_pairs_iter()
            .map(|(luck, unhook_count)| {
                let chance_fail: Luck = 1.0 - luck;
                let chance_fail_all = chance_fail.powi(i32::from(unhook_count) + 3);
                let chance_succeed_once = 1.0 - chance_fail_all;
                (luck, chance_succeed_once)
            })
    }
}

impl std::ops::Add for &TeamLuckRecord {
    type Output = TeamLuckRecord;

    fn add(self, other: Self) -> Self::Output {
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::constants::*;
    use float_cmp::assert_approx_eq;

    const EPSILON_FOUR_SIG_DIGITS: f64 = 0.00001;

    fn altruistic_team() -> TeamLuckRecord {
        let mut personals = ArrayVec::new();
        for _ in 0..3 {
            personals.push((perk_luck::SM_TIER3, 3))
        }

        let global_team_luck_record = TeamLuckRecord::from_global(misc::BASE_UNHOOK_CHANCE);

        &global_team_luck_record + (&TeamLuckRecord {
            global: perk_luck::UTA_TIER3 * 3.0 * 3.0 + offering_luck::GREAT_LUCK * 3.0,
            personals,
        })
    }

    #[test]
    fn trivial_default_comparison() {
        let a = LoadoutLuckRecord::default();
        let b = LoadoutLuckRecord::default();
        assert_eq!(a, b);
    }

    #[test]
    fn ante_prefers_left() {
        let a = LoadoutLuckRecord::from_uta(0.03);
        let b = LoadoutLuckRecord::from_uta(0.02);
        let c = &a + &b;
        assert_eq!(a, c)
    }

    // Note that this test is impossible
    #[test]
    fn integrated_combine() {
        let a = LoadoutLuckRecord {
            personal: 0.04,
            global: 0.01,
            up_the_ante_coeff: Some(0.02),
            additional_unhooks: 0,
        };
        let b = LoadoutLuckRecord {
            personal: 0.02,
            global: 0.02,
            up_the_ante_coeff: None,
            additional_unhooks: 3,
        };
        let c = &a + &b;
        assert_approx_eq!(f64, c.personal, 0.06, epsilon = EPSILON_FOUR_SIG_DIGITS);
        assert_approx_eq!(f64, c.global, 0.03, epsilon = EPSILON_FOUR_SIG_DIGITS);
        assert_approx_eq!(
            f64,
            c.up_the_ante_coeff.unwrap(),
            0.02,
            epsilon = EPSILON_FOUR_SIG_DIGITS
        );
        assert_eq!(c.additional_unhooks, 3)
    }

    #[test]
    fn best_case_integration() {
        let mut personals = ArrayVec::new();
        personals.push((0.04, 3)); // slippery meat
        let player = TeamLuckRecord {
            global: 0.03 + 0.03 * 3., // salty lips & up the ante with 3 others living
            personals,
        };
        let full_team = &altruistic_team() + &player;
        let full_luck: Vec<(Luck, Luck)> = full_team.make_single_and_total_unhook_pairs().collect();
        let (one_try, all_tries) = full_luck.get(3).expect("3 less than full team size");
        assert_approx_eq!(
            f64,
            *all_tries,
            0.992743686,
            epsilon = EPSILON_FOUR_SIG_DIGITS
        );
        assert_approx_eq!(f64, *one_try, 0.5600, epsilon = EPSILON_FOUR_SIG_DIGITS)
    }
}
