use hook_escape_calculator::*;

mod message;
use message::*;
mod widget_data;
use widget_data::*;
pub mod update;
pub mod view;

#[derive(Debug, Clone)]
pub struct App {
    team: team::Team,
    widgets: WidgetData,
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
