use hook_escape_calculator::team;

mod message;
use message::*;
mod widget_data;
use widget_data::*;
pub mod subscription;
pub mod update;
pub mod view;
use iced::window;
use iced::Task;

#[derive(Debug, Clone)]
pub struct App {
    calculator: (iced::window::Id, Calculator),
    help: Option<iced::window::Id>,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let main_window_size = iced::Size::new(1054., 384.);

        let main_window_settings = window::Settings {
            size: main_window_size,
            resizable: false,
            ..window::Settings::default()
        };

        let (id, open) = window::open(main_window_settings);

        (
            App {
                calculator: (id, Calculator::default()),
                help: None,
            },
            open.map(|_| Message::StartApp),
        )
    }
    pub fn title(&self, id: window::Id) -> String {
        let title = if id == self.calculator.0 {
            "Hook Calculator"
        } else if Some(id) == self.help {
            "Hook Calculator -- Help"
        } else {
            "Hook Calculator -- Other"
        };
        title.into()
    }
}

#[derive(Debug, Clone)]
pub struct Calculator {
    team: team::Team,
    widgets: WidgetData,
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
