use arrayvec::ArrayVec;
use derive_getters::Getters;
use itertools::Either;

use super::{
    living_count::LivingCount,
    constants::misc::TEAM_MAX_CAPACITY
};

pub type Luck = f64;

/// A record that represents a players luck items such that
/// two personal lucks are summed rather than list appended
#[derive(PartialEq, Debug, Clone, Copy, Getters)]
pub struct LoadoutLuckRecord {
    personal: Luck,
    global: Luck,
    up_the_ante_coeff: Option<Luck>,
    additional_unhooks: i8,
}

/// Init methods
impl LoadoutLuckRecord {
    const fn const_default() -> Self {
        Self {
            personal: 0.0,
            global: 0.0,
            up_the_ante_coeff: None,
            additional_unhooks: 0
        }
    }
    pub const fn with_personal(personal: Luck) -> Self {
        Self {
            personal,
            ..Self::const_default()
        }
    }
    pub const fn with_global(global: Luck) -> Self {
        Self {
            global,
            ..Self::const_default()
        }
    }
    pub const fn with_uta(uta: Luck) -> Self {
        Self {
            up_the_ante_coeff: Some(uta),
            ..Self::const_default()
        }
    }
    pub const fn with_unhook_mod(additional_unhooks: i8) -> Self {
        Self {
            additional_unhooks,
            ..Self::const_default()
        }
    }
}

impl std::default::Default for LoadoutLuckRecord {
    fn default() -> Self {
        Self::const_default()
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
    pub const fn convert(&self, loadout: LoadoutLuckRecord) -> PlayerLuckRecord {
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
            false => None,
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
    living_other_than_self_count: LivingCount,
}

impl PlayerTeamConverter {
    pub const fn new(living_other_than_self_count: LivingCount) -> Self {
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

        let uta_contribution = up_the_ante_coeff.map_or(0.0, |x| {
            x * f64::from(self.living_other_than_self_count.into_inner())
        });

        let final_global = global + uta_contribution;
        let personal_data = {
            let mut personal_data = ArrayVec::new();
            personal_data.push((personal, additional_unhooks));
            personal_data
        };

        TeamLuckRecord {
            global: final_global,
            personals: Some(personal_data),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeamLuckRecord {
    global: Luck,
    personals: Option<ArrayVec<(Luck, i8), { TEAM_MAX_CAPACITY }>>
}

impl TeamLuckRecord {
    const fn const_default() -> Self {
        TeamLuckRecord {
            global: 0.0,
            personals: None
        }
    }
    pub const fn with_global(luck: Luck) -> Self {
        TeamLuckRecord {
            global: luck,
            personals: None
        }
    }
    pub fn luck_unhook_mod_pairs_iter(&self) -> impl Iterator<Item = (Luck, i8)> + '_ {
        match &self.personals {
            Some(personals) => Either::Left(personals.iter().map(|(l, u)| (l + self.global, *u))),
            None => Either::Right(std::iter::empty::<(f64, i8)>()),
        }
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

impl std::default::Default for TeamLuckRecord {
    fn default() -> Self {
        Self::const_default()
    }
}

impl std::ops::Add for &TeamLuckRecord {
    type Output = TeamLuckRecord;

    fn add(self, other: Self) -> Self::Output {
        let personals = match (&self.personals, &other.personals) {
            (None, None) => None,
            (Some(x), None) | (None, Some(x)) => Some(x.clone()),
            (Some(left), Some(right)) => {
                let mut left: ArrayVec<(f64, i8), TEAM_MAX_CAPACITY> = left.clone();
                left.extend(right.clone());
                Some(left)
        }
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
    use proptest::prelude::*;

    const EPSILON_FOUR_SIG_DIGITS: f64 = 0.00001;

    /// arb is a module that governs how to create and compose arbitrary
    /// inputs, but it does not perform any tests.
    mod arb {
        use super::*;
        prop_compose! {
            pub fn prob()(luck in 0.0_f64..1.0_f64) -> f64 {
                luck
            }
        }
        pub fn maybe_prob() -> impl Strategy<Value = Option<f64>> {
            prop_oneof![Just(None), prob().prop_map(Option::Some),]
        }
        prop_compose! {
            pub fn loadout_record()
                (personal in prob(),
                global in prob(),
                up_the_ante_coeff in maybe_prob(),
                additional_unhooks in 0_i8..100)
                -> LoadoutLuckRecord {
                    LoadoutLuckRecord { personal, global, up_the_ante_coeff, additional_unhooks }
                }
        }
    }

    mod loadout_tests {
        use super::*;

        fn loadout_records_not_overflow(xs: &[LoadoutLuckRecord]) -> bool {
            (-128_i16..128_i16).contains(&xs.iter().map(|x| x.additional_unhooks() as i16).sum())
        }

        proptest! {
            #[test]
            fn add_personals(a in arb::loadout_record(), b in arb::loadout_record()) {
                prop_assume! (loadout_records_not_overflow(&[a, b]));
                let c = &a + &b;
                assert_eq!(*c.personal(), a.personal() + b.personal())
            }
        }
        proptest! {
            #[test]
            fn add_globals(a in arb::loadout_record(), b in arb::loadout_record()) {
                prop_assume! (loadout_records_not_overflow(&[a, b]));
                let c = &a + &b;
                assert_eq!(*c.global(), a.global() + b.global())
            }
        }
        proptest! {
            #[test]
            fn add_unhook_modifier(a in arb::loadout_record(), b in arb::loadout_record()) {
                prop_assume! (loadout_records_not_overflow(&[a, b]));
                let c = &a + &b;
                assert_eq!(c.additional_unhooks(), a.additional_unhooks() + b.additional_unhooks())
            }
        }
        proptest! {
            #[test]
            fn add_uta(a in arb::loadout_record(), b in arb::loadout_record()) {
                prop_assume! ( loadout_records_not_overflow(&[a, b]));
                let c = &a + &b;
                assert_eq!(c.up_the_ante_coeff, a.up_the_ante_coeff.or(b.up_the_ante_coeff))
            }
        }
        proptest! {
            #[test]
            fn add_associative(a in arb::loadout_record(), b in arb::loadout_record(), c in arb::loadout_record()) {
                prop_assume! (loadout_records_not_overflow(&[a, b, c]));
                let (a, b, c) = (&a, &b, &c);
                let (left, right) = (&(a + b) + c, a + &(b + c));
                let p_test = |&l, &r| assert_approx_eq!(f64, l, r);

                p_test(left.global(), right.global());
                p_test(left.personal(), right.personal());
                assert_eq!(left.up_the_ante_coeff(), right.up_the_ante_coeff());
                assert_eq!(left.additional_unhooks(), right.additional_unhooks())
            }
        }
        proptest! {
            #[test]
            fn ante_prefers_left(a in 0.0_f64..1.0_f64, b in 0.0_f64..1.0) {
                let left = LoadoutLuckRecord {
                    up_the_ante_coeff: Some(a),
                    ..LoadoutLuckRecord::default()
                };
                let right = LoadoutLuckRecord {
                    up_the_ante_coeff: Some(b),
                    ..LoadoutLuckRecord::default()
                };
                assert_eq!(left, &left + &right)
            }
        }
        proptest! {
            #[test]
            fn equality(a in arb::loadout_record()) {
                let x = a;
                let y = a;
                assert_eq!(&x, &y, "testing PartialEq with {:?} {:?}", &x, &y);
                assert_eq!(&y, &x, "testing Eq after PartialEq success with {:?} {:?}", &y, &x)
            }
        }
    }
    fn altruistic_team() -> TeamLuckRecord {
        let mut personals = ArrayVec::new();
        for _ in 0..3 {
            personals.push((perk_luck::SM_TIER3, 3))
        }

        let global_team_luck_record = TeamLuckRecord::with_global(misc::BASE_UNHOOK_CHANCE);

        &global_team_luck_record
            + (&TeamLuckRecord {
                global: perk_luck::UTA_TIER3 * 3.0 * 3.0 + offering_luck::GREAT_LUCK * 3.0,
                personals: Some(personals),
            })
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
            personals: Some(personals),
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
