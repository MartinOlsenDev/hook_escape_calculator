use super::living_count::LivingCount;
use super::luck_record::{
    LoadoutLuckRecord, PlayerLuckRecord, PlayerTeamConverter, TeamLuckRecord,
};
use super::player::Player;
use frunk::monoid::combine_all;

const BASE_UNHOOK_CHANCE: f64 = 0.04;
const BASE_UNHOOK_RECORD: PlayerLuckRecord = PlayerLuckRecord(LoadoutLuckRecord::from_global(0.04));

#[derive(Debug, Clone, Copy, Default)]
pub struct Team([Player; 4]);

// Accessor Methods
impl Team {
    pub fn list(&self) -> &[Player; 4] {
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

    fn make_player_luck_records(&self) -> Vec<PlayerLuckRecord> {
        self.list()
            .iter()
            .map(|player| player.make_player_luck())
            .collect()
    }

    fn make_team_adapters(&self) -> Vec<PlayerTeamConverter> {
        self.list()
            .iter()
            .map(|player| self.alive_not_counting(&player))
            .map(|count| PlayerTeamConverter::new(count.into()))
            .collect()
    }

    fn make_team_luck_records(&self) -> Vec<TeamLuckRecord> {
        let player_record_iter = self.make_player_luck_records().into_iter();
        let player_converter_iter = self.make_team_adapters().into_iter();
        let iter = player_record_iter.zip(player_converter_iter);
        iter.map(|(record, converter)| converter.convert(&record))
            .collect()
    }

    fn collate_luck(&self) -> TeamLuckRecord {
        combine_all(&self.make_team_luck_records())
    }

    fn full_make_player_luck(&self) -> [f64; 4] {
        let global_luck = self.calc_global_luck();
        let mut iter = self
            .list()
            .iter()
            .map(|player| player.loadout.make_personal_luck() + global_luck + BASE_UNHOOK_CHANCE);

        [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ]
    }

    pub fn make_escape_chances(&self) -> [f64; 4] {
        let lucks = self.full_make_player_luck();

        let mut iter = self
            .list()
            .iter()
            .map(|player| player.make_max_unhook())
            .zip(lucks)
            .map(|(tries, luck)| {
                let chance_fail: f64 = 1.0 - luck;
                let chance_fail_all = chance_fail.powi(i32::from(tries));
                let chance_succeed_once = 1.0 - chance_fail_all;
                chance_succeed_once
            });
        [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ]
    }
}

// todo: Make Team tests
