mod ui;
use iced::Size;
use ui::App;

fn main() -> iced::Result {
    iced::application("Hook Calculator", App::update, App::view)
        .theme(App::theme)
        .window_size(Size::new(1054., 384.))
        .resizable(false)
        .run()
}
