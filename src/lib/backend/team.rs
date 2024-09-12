use super::living_count::LivingCount;
use super::player::Player;
use crate::lib::backend::luck::{CalculatedLuck, DynamicLuck};

const BASE_UNHOOK_CHANCE: f64 = 0.04;

#[derive(Debug, Clone, Copy, Default)]
pub struct Team([Player; 4]);

// Modification Methods
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
                .filter(|&player| player.is_alive)
                .filter(|&player| !std::ptr::eq(player, uncounted_player))
                .count() as u8,
        )
        .expect("Filtering on a [_;4] cannot yield count exceeding 4.")
    }

    fn calc_global_static_luck(&self) -> f64 {
        self.list()
            .iter()
            .map(|player| player.loadout.make_global_luck())
            .sum()
    }

    pub fn calc_global_dyn_luck(&self) -> f64 {
        self.list()
            .iter()
            .map(|player| player.make_ante_luck(&self))
            .sum()
    }

    fn full_make_player_luck(&self) -> [f64; 4] {
        let global_luck = self.calc_global_static_luck() + self.calc_global_dyn_luck();
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
            .zip(lucks)
            .map(|(player, luck)| (player.loadout.make_max_unhook(), luck))
            .map(|(tries, luck)| {
                let chance_fail: f64 = 1.0 - luck;
                let chance_fail_all = chance_fail.powi(tries as i32);
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
