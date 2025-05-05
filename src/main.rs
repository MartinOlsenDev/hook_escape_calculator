mod lib;
mod ui;
use iced;
use ui::App;

fn main() -> iced::Result {
    iced::application("Hook Calculator", App::update, App::view)
        //.theme(App::theme)
        .run()
}
