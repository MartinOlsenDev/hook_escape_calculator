use std::borrow::Cow;

use arrayvec::ArrayVec;
use iced::widget::combo_box;

use hook_escape_calculator::{constants::misc as k, offering, perk, team};

#[derive(Debug, Clone)]
pub struct WidgetData {
    pub tier_choices: combo_box::State<TierSlotDisplay>,
    pub offering_choices: combo_box::State<OfferingSlotDisplay>,
    pub odds: ArrayVec<(String, String), { k::TEAM_MAX_CAPACITY }>,
}

impl WidgetData {
    pub fn renew_odds(&mut self, team: &team::Team) {
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
pub struct OfferingSlotDisplay(pub offering::OfferingSlot);

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
