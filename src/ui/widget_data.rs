use std::borrow::Cow;

use iced::widget::combo_box;

use hook_escape_calculator::{
    offering::{Offering, OfferingSlot},
    perk, team,
};

#[derive(Debug, Clone)]
pub struct WidgetData {
    pub tier_choices: combo_box::State<TierSlotDisplay>,
    pub offering_choices: combo_box::State<OfferingSlotDisplay>,
    pub odds: Vec<(String, String)>,
}

impl WidgetData {
    pub fn from_team(team: &team::Team) -> Self {
        let tier_choices = TierSlotDisplay::total_combo_box();
        let offering_choices = OfferingSlotDisplay::total_combo_box();
        let odds = Self::make_odds(team);
        Self {
            tier_choices,
            offering_choices,
            odds
        }
    }

    pub fn renew_odds(&mut self, team: &team::Team) {
        self.odds = Self::make_odds(team);
    }

    fn make_odds(team: &team::Team) -> Vec<(String, String)> {
        let f = |num: f64| {
            let num = num * 100.;
            format!("{num:.2}%")
        };

        team
            .luck_output()
            .into_iter()
            .map(|(num1, num2)| (f(num1), f(num2)))
            .collect()
    }
}

type TierSlot = Option<perk::Tier>;
#[derive(Debug, Clone)]
pub struct TierSlotDisplay(pub TierSlot);

impl TierSlotDisplay {
    pub fn total_combo_box() -> combo_box::State<Self> {
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
pub struct OfferingSlotDisplay(pub OfferingSlot);

impl OfferingSlotDisplay {
    pub fn total_combo_box() -> combo_box::State<Self> {
        combo_box::State::new(
            Offering::iterator()
                .map(|x| OfferingSlotDisplay(OfferingSlot::new(Some(x))))
                .chain(std::iter::once(OfferingSlotDisplay(OfferingSlot::new(
                    None,
                ))))
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
