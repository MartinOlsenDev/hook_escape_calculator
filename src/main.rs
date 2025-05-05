mod lib;
mod ui;
use ui::App;
use iced;

fn main() -> iced::Result {
    iced::application("Hook Calculator", App::update, App::view)
        //.theme(App::theme)
        .run()
}