use iced::{Size, Task, window};

use hook_escape_calculator::team;

use super::{
    message::Message,
    widget_data::{OfferingSlotDisplay, TierSlotDisplay, WidgetData},
};

#[derive(Debug, Clone)]
pub struct App {
    pub calculator: Calculator,
    pub main_window: window::Id,
    pub help_window: Option<iced::window::Id>,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let main_window_size = Size::new(1054., 384.);

        let main_window_settings = window::Settings {
            size: main_window_size,
            resizable: false,
            ..window::Settings::default()
        };

        let (id, open) = window::open(main_window_settings);

        (
            App {
                calculator: Calculator::default(),
                main_window: id,
                help_window: None,
            },
            open.map(|_| Message::StartApp),
        )
    }
    pub fn title(&self, id: window::Id) -> String {
        let title = if id == self.main_window {
            "Hook Calculator"
        } else if Some(id) == self.help_window {
            "Hook Calculator \u{2012} Help"
        } else {
            "Hook Calculator \u{2012} Other"
        };
        title.into()
    }
}

#[derive(Debug, Clone)]
pub struct Calculator {
    pub team: team::Team,
    pub widgets: WidgetData,
}

impl std::default::Default for Calculator {
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
        Calculator { team, widgets }
    }
}
