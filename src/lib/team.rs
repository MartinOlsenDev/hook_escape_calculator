use super::living_count::LivingCount;
use super::luck_record::{PlayerTeamConverter, TeamLuckRecord};
use super::player::Player;
use arrayvec::ArrayVec;

use crate::constants::misc as k;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Team([Player; k::TEAM_MAX_CAPACITY]);

// Mutable Accessor Methods
impl Team {
    pub fn list(&self) -> &[Player; k::TEAM_MAX_CAPACITY] {
        &self.0
    }
    pub fn get_player_mut(&mut self, i: usize) -> Option<&mut Player> {
        self.0.get_mut(i)
    }
    pub fn get_player(&self, i: usize) -> Option<&Player> {
        self.0.get(i)
    }
}

// Calculating Methods
impl Team {
    fn alive_not_counting(self, uncounted_player: usize) -> LivingCount {
        LivingCount::try_from(
            u8::try_from(
                self.list()
                    .iter()
                    .enumerate()
                    .filter(|(_, player)| player.is_alive())
                    .filter(|(i, _)| *i != uncounted_player)
                    .count(),
            )
            .expect("Filtering on [_;4] cannot yield count outside u8"),
        )
        .expect("Filtering on a [_;4] cannot yield count exceeding 4.")
    }

    fn make_team_luck_records(&self) -> impl Iterator<Item = TeamLuckRecord> + '_ {
        let player_record_iter = self.list().iter().map(|player| player.make_player_luck());
        let player_converter_iter = (0_usize..(self.list().len()))
            .map(|id| self.alive_not_counting(id))
            .map(|count| PlayerTeamConverter::new(count.into()));

        player_record_iter
            .zip(player_converter_iter)
            .map(|(record, converter)| converter.convert(&record))
    }

    fn collate_luck(&self) -> TeamLuckRecord {
        let base_luck: TeamLuckRecord = TeamLuckRecord::from_global(k::BASE_UNHOOK_CHANCE);
        let team_luck_records = self.make_team_luck_records();

        team_luck_records.fold(base_luck, |acc, x| &acc + &x)
    }

    pub fn luck_output(&self) -> ArrayVec<(f64, f64), { k::TEAM_MAX_CAPACITY }> {
        self.collate_luck()
            .make_single_and_total_unhook_pairs()
            .collect()
    }
}
