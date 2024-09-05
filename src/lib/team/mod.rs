pub mod living_count;
use living_count::LivingCount;
use super::player::Loadout;
use super::player::Player;

const BASE_UNHOOK_CHANCE: f64 = 0.04;


pub struct Team([Player; 4]);

// Modification Methods
impl Team {
    fn list(&self) -> &[Player; 4] {
        &self.0
    }
}

// Calculating Methods
impl Team {
    fn alive_not_counting(&self, uncounted_player: &Player) -> LivingCount {
        /// Counting a list of 4 members that has been filtered cannot possibly exceed 4,
        /// So LivingCount::try_from is infallible in this case.
        LivingCount::try_from(
            self.list().iter()
            .filter(|&player| player.is_alive)
            .filter(|&player| !std::ptr::eq(player, uncounted_player))
            .count()
            as u8
        ).unwrap()
    }

    fn calc_global_static_luck(&self) -> f64 {
        self.list().iter()
            .map(|player| player.loadout.global_static_modifier())
            .sum()
    }

    fn calc_global_dyn_luck(&self) -> f64 {
        self.list().iter()
            .filter(|player| player.is_alive)
            .map(|player| (player, (self.alive_not_counting(&player))))
            .map(|(player, living_count)| player.loadout.ante_calculator(living_count))
            .sum()
    }

    fn full_make_player_luck(&self) -> [f64; 4] {
        let global_luck = self.calc_global_static_luck() + self.calc_global_dyn_luck();
        let mut iter = self.list().iter()
            .map(|player|
                player.loadout.make_personal_luck()
                + global_luck
                + BASE_UNHOOK_CHANCE
            );

        [iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap()
        ]
    }

    pub fn make_escape_chances(&self) -> [f64; 4] {
        let lucks = self.full_make_player_luck();

        let mut iter = self.list().iter().zip(lucks)
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
            iter.next().unwrap()
        ]
    }
}

impl Default for Team {
    fn default() -> Self {
        todo!()
    }
}
