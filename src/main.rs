mod ui;
use ui::App;

fn main() -> iced::Result {
    iced::daemon(App::title, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .run_with(App::new)
}
