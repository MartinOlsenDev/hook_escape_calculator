use super::{
    constants::misc as k,
    living_count::{LivingCount, LivingCountError},
    luck_record::{PlayerTeamConverter, TeamLuckRecord},
    player::Player,
    update::{SurvivorId, SurvivorUpdate},
};

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Team([Player; k::TEAM_MAX_CAPACITY]);

// Accessor Methods
impl Team {
    pub fn list(&self) -> impl Iterator<Item = &Player> + '_ {
        self.0.iter()
    }
    pub fn get_player(&self, i: SurvivorId) -> &Player {
        self.0
            .get(*i)
            .expect("SurvivorId always valid for Team size.")
    }
}

// Mutating Methods
impl Team {
    fn get_player_mut(&mut self, i: SurvivorId) -> &mut Player {
        self.0
            .get_mut(*i)
            .expect("SurvivorId always valid for Team size.")
    }
    pub fn alter(&mut self, update: SurvivorUpdate) {
        let player_to_change = self.get_player_mut(*update.id());
        player_to_change.alter(*update.update());
    }
}

// Calculating Methods
impl Team {
    fn alive_not_counting(&self, uncounted_player: &usize) -> LivingCount {
        let raw_answer = self
            .list()
            .enumerate()
            .filter(|(_, player)| player.is_alive())
            .filter(|(i, _)| i != uncounted_player)
            .count();
        u8::try_from(raw_answer)
            .map_err(|_| LivingCountError::LessOrEqualViolated)
            .and_then(LivingCount::try_new)
            .expect("Cannot generate living count above the max from a list of size max.")
    }

    fn make_team_luck_records(&self) -> impl Iterator<Item = TeamLuckRecord> + '_ {
        let player_record_iter = self.list().map(|player| player.make_player_luck());
        let player_converter_iter = (0_usize..(self.0.len()))
            .map(|id| self.alive_not_counting(&id))
            .map(PlayerTeamConverter::new);

        player_record_iter
            .zip(player_converter_iter)
            .map(|(record, converter)| converter.convert(&record))
    }

    fn collate_luck(&self) -> TeamLuckRecord {
        let base_luck: TeamLuckRecord = TeamLuckRecord::with_global(k::BASE_UNHOOK_CHANCE);
        let team_luck_records = self.make_team_luck_records();

        team_luck_records.fold(base_luck, |acc, x| &acc + &x)
    }

    pub fn luck_output(&self) -> Vec<(f64, f64)> {
        let mut output: Vec<(f64, f64)> = Vec::with_capacity(k::TEAM_MAX_CAPACITY);

        self.collate_luck()
            .make_single_and_total_unhook_pairs()
            .for_each(|x| output.push(x));
        output
    }
}

#[cfg(test)]
pub mod arb {
    use super::super::player;
    use super::*;
    use proptest::prelude::*;

    prop_compose! {
        pub fn team()(players in prop::collection::vec(player::arb::player(), k::TEAM_MAX_CAPACITY)) -> Team {
            let mut team = [Player::default(); k::TEAM_MAX_CAPACITY];

            for (i, player) in players.into_iter().take(k::TEAM_MAX_CAPACITY).enumerate() {
                team[i] = player;
            }

            Team(team)
        }
    }

    pub fn collate_luck_cfg_test(t: &Team) -> TeamLuckRecord {
        t.collate_luck()
    }
}

#[cfg(test)]
mod tests {
    use super::super::constants::observations as obs;
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn no_player_single_try_less_than_min_single_luck(team in arb::team()) {
            let lucks = team.luck_output();
            let all_single_lucks: Vec<f64> = lucks.into_iter().map(|(single, _)| single).collect();
            let single_lucks_gte_min: Vec<bool> = all_single_lucks.into_iter().map(|luck| luck >= obs::MIN_SINGLE_LUCK).collect();
            let all_single_lucks_gte_min: bool = single_lucks_gte_min.into_iter().all(|x| x);


            prop_assert!(all_single_lucks_gte_min)
        }
    }

    proptest! {
        #[test]
        fn no_player_single_try_more_than_max_single_luck(team in arb::team()) {
            let lucks = team.luck_output();
            let all_single_lucks: Vec<f64> = lucks.into_iter().map(|(single, _)| single).collect();
            let single_lucks_lte_min: Vec<bool> = all_single_lucks.into_iter().map(|luck| luck <= obs::MAX_SINGLE_LUCK).collect();
            let all_single_lucks_lte_min: bool = single_lucks_lte_min.into_iter().all(|x| x);


            prop_assert!(all_single_lucks_lte_min)
        }
    }
}
