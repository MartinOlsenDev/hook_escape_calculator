use fltk::{prelude::*, *};

mod lib;
mod ui;

fn main() {
    let app = app::App::default();
    let mut ui = ui::UserInterface::make_window();
    /*ui.but.set_callback(move |_| {
        println!("Works!");
    });*/
    app.run().expect("Internal fltk error encountered.");
}
