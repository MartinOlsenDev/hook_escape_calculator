use super::living_count::LivingCount;
use super::luck_record::{
    LoadoutLuckRecord, PlayerLuckRecord, PlayerTeamConverter, TeamLuckRecord,
};
use super::player::Player;
use arrayvec::ArrayVec;
use frunk::monoid::combine_all;
use frunk::Semigroup;

const BASE_UNHOOK_CHANCE: f64 = 0.04;
pub const TEAM_MAX_CAPACITY: usize = 4;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Team([Player; TEAM_MAX_CAPACITY]);

// Accessor Methods
impl Team {
    pub fn list(&self) -> &[Player; TEAM_MAX_CAPACITY] {
        &self.0
    }
}

// Calculating Methods
impl Team {
    fn alive_not_counting(&self, uncounted_player: &Player) -> LivingCount {
        LivingCount::try_from(
            self.list()
                .iter()
                .filter(|&player| player.is_alive())
                .filter(|&player| !std::ptr::eq(player, uncounted_player))
                .count() as u8,
        )
        .expect("Filtering on a [_;4] cannot yield count exceeding 4.")
    }

    fn make_player_luck_records<'a>(&'a self) -> impl Iterator<Item = PlayerLuckRecord> + 'a {
        self.list().iter().map(|player| player.make_player_luck())
    }

    fn make_team_adapters<'a>(&'a self) -> impl Iterator<Item = PlayerTeamConverter> + 'a {
        self.list()
            .iter()
            .map(|player| self.alive_not_counting(&player))
            .map(|count| PlayerTeamConverter::new(count.into()))
    }

    fn make_team_luck_records<'a>(&'a self) -> impl Iterator<Item = TeamLuckRecord> + 'a {
        let player_record_iter = self.make_player_luck_records().into_iter();
        let player_converter_iter = self.make_team_adapters().into_iter();
        let iter = player_record_iter.zip(player_converter_iter);
        iter.map(|(record, converter)| converter.convert(&record))
    }

    fn collate_luck(&self) -> TeamLuckRecord {
        let base_luck: TeamLuckRecord = TeamLuckRecord::from_global(BASE_UNHOOK_CHANCE);
        let team_luck_records: ArrayVec<TeamLuckRecord, TEAM_MAX_CAPACITY> =
            self.make_team_luck_records().collect();
        base_luck.combine(&combine_all(&team_luck_records))
    }

    pub fn luck_string_output(&self) -> ArrayVec<(String, String), TEAM_MAX_CAPACITY> {
        self.collate_luck().make_single_and_total_unhook_strings()
    }
}
