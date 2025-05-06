use crate::lib::*;

use std::borrow::Cow;

use arrayvec::ArrayVec;
use iced::widget::{checkbox, combo_box, row, text, Column, Text};
use iced::Element;

use crate::lib::constants as k;

#[derive(Debug, Clone)]
pub struct App {
    team: team::Team,
    widgets: WidgetData,
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Unimplemented => eprintln!("Unimplemented"),
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

    pub fn view(&self) -> Element<Message> {
        self.view_team()
    }

    fn view_team(&self) -> Element<Message> {
        let mut rows = Column::new();
        let name_header = text("Survivor Name").width(120);
        let input_headers = row![
            text("Slippery Meat").width(200),
            text("Up the Ante").width(200),
            text("Offering").width(200),
            text("Living Status").width(120)
        ];
        let output_headers = row![
        text("Attempt Chance").width(120),
        text("Total Chance").width(120)
        ].width(240);
        let column_headers = row![
            name_header,
            input_headers,
            output_headers
        ];


        rows = rows.push(column_headers);

        for player_id in 0..k::TEAM_MAX_CAPACITY {
            let player_name = text(format!("Player {}", player_id + 1)).width(120);
            let row_input = self.make_player(player_id);
            let (attempt_chance, total_chance) = self.widgets.odds.get(player_id).expect(
                "Generated id in range 0..TEAM_MAX_CAPACITY always less than TEAM_MAX_CAPACITY.",
            );
            let row_output = row![
                text(attempt_chance.to_owned()).width(120),
                text(total_chance.to_owned()).width(120)
            ];
            let row = row![player_name, row_input, row_output];
            rows = rows.push(row)
        }
        rows.into()
    }

    fn make_player(&self, id: usize) -> Element<Message> {
        let player = self.team.get_player(id).expect(
            "Generated id in range 0..TEAM_MAX_CAPACITY always less than TEAM_MAX_CAPACITY.",
        );

        row![
            combo_box(
                &self.widgets.tier_choices,
                "",
                Some(&TierSlotDisplay(player.get_slippery_tier())),
                move |TierSlotDisplay(x)| {
                    Message::UpdateSurvivor(SurvivorUpdate::slippery(id, x))
                }
            ).width(200),
            combo_box(
                &self.widgets.tier_choices,
                "",
                Some(&TierSlotDisplay(player.get_uta_tier())),
                move |TierSlotDisplay(x)| { Message::UpdateSurvivor(SurvivorUpdate::uta(id, x)) }
            ).width(200),
            combo_box(
                &self.widgets.offering_choices,
                "",
                Some(&OfferingSlotDisplay(player.get_offering())),
                move |OfferingSlotDisplay(x)| {
                    Message::new_surv_update(id, SurvivorUpdateData::Offering(x))
                }
            ).width(200),
            checkbox("is dead", player.is_dead())
                .on_toggle(move |x| Message::new_surv_update(id, SurvivorUpdateData::Life(x)))
                .width(120)
        ]
        .into()
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Dracula
    }
}

#[derive(Debug, Clone)]
struct WidgetData {
    tier_choices: combo_box::State<TierSlotDisplay>,
    offering_choices: combo_box::State<OfferingSlotDisplay>,
    odds: ArrayVec<(String, String), { k::TEAM_MAX_CAPACITY }>,
}

impl WidgetData {
    fn renew_odds(&mut self, team: &team::Team) {
        let f = |num: f64| {
            let num = num * 100.;
            format!("{num:.2}%")
        };

        self.odds = team
            .luck_output()
            .into_iter()
            .map(|(num1, num2)| (f(num1), f(num2)))
            .collect();
    }
}

impl std::default::Default for App {
    fn default() -> Self {
        let team = team::Team::default();
        let widgets = {
            let tier_choices = TierSlotDisplay::total_combo_box();

            let offering_choices = OfferingSlotDisplay::total_combo_box();

            let empty_odds = {
                let f = || "%0.00".to_string();
                core::array::from_fn(|_| (f(), f()))
            };
            let odds = empty_odds.into();

            let mut widgets = WidgetData {
                tier_choices,
                offering_choices,
                odds,
            };
            widgets.renew_odds(&team);
            widgets
        };
        App { team, widgets }
    }
}

type TierSlot = Option<perk::Tier>;
#[derive(Debug, Clone)]
struct TierSlotDisplay(TierSlot);

impl TierSlotDisplay {
    fn total_combo_box() -> combo_box::State<Self> {
        combo_box::State::new(
            perk::Tier::iterator()
                .map(|x| TierSlotDisplay(Some(x)))
                .chain(std::iter::once(TierSlotDisplay(None)))
                .collect(),
        )
    }
}

impl std::fmt::Display for TierSlotDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: Cow<_> = self
            .0
            .map(|x| Cow::Owned(x.to_string().to_uppercase()))
            .unwrap_or(Cow::Borrowed("NA"));
        write!(f, "{}", &s)
    }
}

#[derive(Debug, Clone)]
struct OfferingSlotDisplay(offering::OfferingSlot);

impl OfferingSlotDisplay {
    pub fn total_combo_box() -> combo_box::State<Self> {
        combo_box::State::new(
            offering::Offering::iterator()
                .map(|x| OfferingSlotDisplay(Some(x)))
                .chain(std::iter::once(OfferingSlotDisplay(None)))
                .collect(),
        )
    }
}

impl std::fmt::Display for OfferingSlotDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: Cow<_> = self
            .0
            .map(|x| Cow::Owned(x.to_string().to_uppercase()))
            .unwrap_or(Cow::Borrowed("NA"));
        write!(f, "{}", &s)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Message {
    UpdateSurvivor(SurvivorUpdate),
    Unimplemented,
}

impl Message {
    fn new_surv_update(id: usize, update: SurvivorUpdateData) -> Message {
        Message::UpdateSurvivor(SurvivorUpdate { id, update })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct SurvivorUpdate {
    id: usize,
    update: SurvivorUpdateData,
}

impl SurvivorUpdate {
    fn slippery(id: usize, tier: Option<perk::Tier>) -> SurvivorUpdate {
        SurvivorUpdate {
            id,
            update: SurvivorUpdateData::Perk(PerkUpdate {
                perk: perk::PerkName::SlipperyMeat,
                value: tier,
            }),
        }
    }
    fn uta(id: usize, tier: Option<perk::Tier>) -> SurvivorUpdate {
        SurvivorUpdate {
            id,
            update: SurvivorUpdateData::Perk(PerkUpdate {
                perk: perk::PerkName::UpTheAnte,
                value: tier,
            }),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SurvivorUpdateData {
    Perk(PerkUpdate),
    Offering(Option<offering::Offering>),
    Life(bool),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct PerkUpdate {
    perk: perk::PerkName,
    value: Option<perk::Tier>,
}
