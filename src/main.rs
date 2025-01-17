use fltk::{prelude::*, *};

mod lib;
mod ui;

fn main() {
    let app = app::App::default();
    let mut ui = ui::UserInterface::make_window();
    app.run().expect("Internal fltk error encountered.");
}
