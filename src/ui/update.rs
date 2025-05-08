use super::App;
use super::Message;
use super::SurvivorUpdate;
use super::SurvivorUpdateData;
use super::PerkUpdate;

use hook_escape_calculator::perk;

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::UpdateSurvivor(x) => self.update_survivor(x),
        }
    }

    fn update_survivor(&mut self, survivor_update: SurvivorUpdate) {
        let player = self.team.get_player_mut(survivor_update.id).expect(
            "Generated id in range 0..TEAM_MAX_CAPACITY always less than TEAM_MAX_CAPACITY.",
        );

        match survivor_update.update {
            SurvivorUpdateData::Life(false) => player.set_alive(),
            SurvivorUpdateData::Life(true) => player.set_dead(),
            SurvivorUpdateData::Offering(x) => player.set_offering(x),
            SurvivorUpdateData::Perk(PerkUpdate { perk, value }) => match perk {
                perk::PerkName::SlipperyMeat => player.set_slippery(value),
                perk::PerkName::UpTheAnte => player.set_uta(value),
            },
        };

        self.widgets.renew_odds(&self.team);
    }
}