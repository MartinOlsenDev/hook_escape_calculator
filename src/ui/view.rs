use arrayvec::ArrayVec;
use iced::{
    Element, Padding,
    widget::{Column, Row, button, checkbox, column, combo_box, container, row, text},
    window,
};

use hook_escape_calculator::{
    constants::misc as k,
    offering::OfferingSlot,
    perk::{PerkName, TierSlot},
    update::{SurvivorId, SurvivorIdError, SurvivorUpdate},
};

use super::{
    App, Calculator, Message, help_window,
    widget_data::{OfferingSlotDisplay, TierSlotDisplay},
};

impl App {
    pub fn view(&self, window_id: window::Id) -> Element<Message> {
        if window_id == self.main_window {
            self.calculator.view()
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

        let ids: ArrayVec<SurvivorId, { k::TEAM_MAX_CAPACITY }> = (0..k::TEAM_MAX_CAPACITY)
            .map(SurvivorId::try_new)
            .collect::<Result<ArrayVec<SurvivorId, { k::TEAM_MAX_CAPACITY }>, SurvivorIdError>>()
            .expect(
                "Can't observe a value >= max capacity in iterator below bound of max capacity.",
            );

        let make_name =
            |player_id: SurvivorId| text(format!("Player {}", *player_id + 1)).width(125);
        let make_input = |player_id| self.make_player(player_id);
        let make_output = |player_id: SurvivorId| -> Row<'_, Message> {
            let (attempt_chance, total_chance) = self.widgets.odds.get(*player_id).expect(
                "Generated id in range 0..TEAM_MAX_CAPACITY always less than TEAM_MAX_CAPACITY.",
            );
            row![
                container(text(attempt_chance.to_owned()))
                    .padding(Padding::ZERO.left(10))
                    .width(120),
                container(text(total_chance.to_owned()))
                    .padding(Padding::ZERO.left(10))
                    .width(120)
            ]
        };
        let make_row = |id: SurvivorId| {
            container(row![make_name(id), make_input(id), make_output(id)]).height(50)
        };

        ids.into_iter()
            .map(make_row)
            .fold(rows, Column::push)
            .into()
    }

    fn make_player(&self, id: SurvivorId) -> Element<Message> {
        let player = self.team.get_player(id);

        row![
            container(
                combo_box(
                    &self.widgets.tier_choices,
                    "",
                    Some(&TierSlotDisplay(
                        player.get_perk_tier(PerkName::SlipperyMeat).cloned()
                    )),
                    move |TierSlotDisplay(x)| {
                        Message::UpdateSurvivor(
                            SurvivorUpdate::perk()
                                .id(id)
                                .perk(PerkName::SlipperyMeat)
                                .tier(TierSlot::new(x))
                                .call(),
                        )
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
                        Message::UpdateSurvivor(
                            SurvivorUpdate::perk()
                                .id(id)
                                .perk(PerkName::UpTheAnte)
                                .tier(TierSlot::new(x))
                                .call(),
                        )
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
                    Some(&OfferingSlotDisplay(*player.offering())),
                    move |OfferingSlotDisplay(x)| {
                        Message::UpdateSurvivor(
                            SurvivorUpdate::offering()
                                .id(id)
                                .offering(OfferingSlot::new(*x))
                                .call(),
                        )
                    }
                )
                .width(150)
            )
            .padding(Padding::ZERO.left(23))
            .center_x(200),
            container(
                checkbox("", player.is_dead()).on_toggle(move |x| Message::UpdateSurvivor(
                    SurvivorUpdate::living_status().id(id).alive(!x).call()
                ))
            )
            .center_x(120)
        ]
        .into()
    }
}
