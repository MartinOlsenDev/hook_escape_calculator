use hook_escape_calculator::update::SurvivorUpdate;
use iced::window;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Message {
    UpdateSurvivor(SurvivorUpdate),
    OpenHelp,
    CloseHelp,
    ExitApp,
    StartApp,
    CloseWindow(window::Id),
    Noop,
}
