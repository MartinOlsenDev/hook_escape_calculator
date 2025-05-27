use iced::{
    Element, Padding,
    widget::{Column, button, checkbox, column, combo_box, container, row, text},
    window,
};

use super::{
    App, Calculator, Message, OfferingSlotDisplay, SurvivorUpdate, SurvivorUpdateData,
    TierSlotDisplay, help_window,
};

use hook_escape_calculator::constants::misc as k;
use hook_escape_calculator::perk::PerkName;

impl App {
    pub fn view(&self, window_id: window::Id) -> Element<Message> {
        if window_id == self.calculator.0 {
            self.calculator.1.view()
        } else {
            help_window::view()
        }
    }

    pub fn theme(&self, _: window::Id) -> iced::Theme {
        iced::Theme::Dracula
    }
}

impl Calculator {
    pub fn view(&self) -> Element<Message> {
        column![
            container(button("About").on_press(Message::OpenHelp))
                .align_right(1054)
                .align_top(40),
            self.view_team()
        ]
        .into()
    }

    fn view_team(&self) -> Element<Message> {
        let mut rows = Column::new();
        let name_header = container(text("Survivor Name")).align_bottom(30).width(125);
        let input_headers = row![
            container(text("Slippery Meat"))
                .center_x(200)
                .align_bottom(30),
            container(text("Up the Ante"))
                .center_x(200)
                .align_bottom(30),
            container(text("Offering")).center_x(200).align_bottom(30),
            container(text("Is Dead\nStatus")).center_x(120)
        ];
        let output_headers = row![
            text("Attempt\nChance").width(120),
            text("Total\nChance").width(120)
        ]
        .width(240);
        let column_headers = row![name_header, input_headers, output_headers].height(60);

        rows = rows.push(column_headers);

        for player_id in 0..k::TEAM_MAX_CAPACITY {
            let player_name = text(format!("Player {}", player_id + 1)).width(125);
            let row_input = self.make_player(player_id);
            let (attempt_chance, total_chance) = self.widgets.odds.get(player_id).expect(
                "Generated id in range 0..TEAM_MAX_CAPACITY always less than TEAM_MAX_CAPACITY.",
            );
            let row_output = row![
                container(text(attempt_chance.to_owned()))
                    .padding(Padding::ZERO.left(10))
                    .width(120),
                container(text(total_chance.to_owned()))
                    .padding(Padding::ZERO.left(10))
                    .width(120)
            ];
            let row = container(row![player_name, row_input, row_output]).height(50);
            rows = rows.push(row)
        }
        rows.into()
    }

    fn make_player(&self, id: usize) -> Element<Message> {
        let player = self.team.get_player(id).expect(
            "Generated id in range 0..TEAM_MAX_CAPACITY always less than TEAM_MAX_CAPACITY.",
        );

        row![
            container(
                combo_box(
                    &self.widgets.tier_choices,
                    "",
                    Some(&TierSlotDisplay(
                        player.get_perk_tier(PerkName::SlipperyMeat).cloned()
                    )),
                    move |TierSlotDisplay(x)| {
                        Message::UpdateSurvivor(SurvivorUpdate::slippery(id, x))
                    }
                )
                .width(120)
            )
            .padding(Padding::ZERO.left(10))
            .center_x(200),
            container(
                combo_box(
                    &self.widgets.tier_choices,
                    "",
                    Some(&TierSlotDisplay(
                        player.get_perk_tier(PerkName::UpTheAnte).cloned()
                    )),
                    move |TierSlotDisplay(x)| {
                        Message::UpdateSurvivor(SurvivorUpdate::uta(id, x))
                    }
                )
                .width(120)
            )
            .padding(Padding::ZERO.left(12))
            .center_x(200),
            container(
                combo_box(
                    &self.widgets.offering_choices,
                    "",
                    Some(&OfferingSlotDisplay(player.get_offering().cloned())),
                    move |OfferingSlotDisplay(x)| {
                        Message::new_surv_update(id, SurvivorUpdateData::Offering(x))
                    }
                )
                .width(150)
            )
            .padding(Padding::ZERO.left(23))
            .center_x(200),
            container(
                checkbox("", player.is_dead())
                    .on_toggle(move |x| Message::new_surv_update(id, SurvivorUpdateData::Life(x)))
            )
            .center_x(120)
        ]
        .into()
    }
}
