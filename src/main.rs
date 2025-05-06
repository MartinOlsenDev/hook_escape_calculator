mod lib;
mod ui;
use iced;
use iced::Size;
use ui::App;

fn main() -> iced::Result {
    iced::application("Hook Calculator", App::update, App::view)
        .theme(App::theme)
        .window_size(Size::new(1024., 384.))
        .resizable(false)
        .run()
}
