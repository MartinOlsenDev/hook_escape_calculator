use super::{App, Calculator, Message, PerkUpdate, SurvivorUpdate, SurvivorUpdateData};
use iced::window;

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::UpdateSurvivor(x) => self.calculator.1.update_survivor(x),
            Message::ExitApp => std::process::exit(0),
            Message::StartApp => (),
            Message::CloseHelp => self.help = None,
            Message::OpenHelp(id) => self.help = Some(id),
            Message::CloseWindow(id) => self.update(self.specify_close(id)),
        }
    }

    fn specify_close(&self, id: window::Id) -> Message {
        if id == self.calculator.0 {
            Message::ExitApp
        } else {
            Message::CloseHelp
        }
    }
}

impl Calculator {
    fn update_survivor(&mut self, survivor_update: SurvivorUpdate) {
        let player = self.team.get_player_mut(survivor_update.id).expect(
            "Generated id in range 0..TEAM_MAX_CAPACITY always less than TEAM_MAX_CAPACITY.",
        );

        match survivor_update.update {
            SurvivorUpdateData::Life(false) => player.set_alive(),
            SurvivorUpdateData::Life(true) => player.set_dead(),
            SurvivorUpdateData::Offering(x) => player.set_offering(x),
            SurvivorUpdateData::Perk(PerkUpdate { perk, value }) => {
                player.set_perk_tier(perk, value)
            }
        };

        self.widgets.renew_odds(&self.team);
    }
}
