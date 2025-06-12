use arrayvec::ArrayVec;

use super::{
    living_count::{LivingCount, LivingCountError},
    luck_record::{PlayerTeamConverter, TeamLuckRecord},
    player::Player,
    update::{SurvivorId, SurvivorUpdate},
    constants::misc as k
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
        self.list().enumerate().map(|(id, player)| {
            let player_luck = player.make_player_luck();
            let converter = PlayerTeamConverter::new(self.alive_not_counting(&id));
            converter.convert(&player_luck)
        })
    }

    fn collate_luck(&self) -> TeamLuckRecord {
        let base_luck: TeamLuckRecord = TeamLuckRecord::with_global(k::BASE_UNHOOK_CHANCE);
        let team_luck_records = self.make_team_luck_records();

        team_luck_records.fold(base_luck, |acc, x| &acc + &x)
    }

    pub fn luck_output(&self) -> ArrayVec<(f64, f64), { k::TEAM_MAX_CAPACITY }> {
        self.collate_luck()
            .make_single_and_total_unhook_pairs()
            .collect()
    }
}
