#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use cursive;
use cursive::Cursive;
use cursive::views::Dialog;
use cursive::views::TextView;
use tracing::info;

// --- my stuff ---
mod dice_bag;
mod tavern;

use crate::dice_bag::*;
use crate::tavern::structs::List::App;
use crate::tavern::*;

// todo!("add types of mead to the drink list");

// --- local cli code
fn main() -> () {
    let mut app: App = App::new();
    app.name = "fantasy-tavern-maker-tui".into();
    app.version_build = 11;
    app.version_major = 0;
    app.version_minor = 6;
    app.version_fix = 11;

    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text(&format!("Welcome to {}", &app.name))
            .title(&app.name)
            .button("New", |s| get_new_pbhouse(s))
            .button("Finish", |s| s.quit()),
    );

    siv.run()
}

fn get_new_pbhouse(s: &mut Cursive) -> () {
    s.pop_layer();
    s.add_layer(
        Dialog::text(get_pbhouse())
            .title("Results")
            .button("Another", |s| get_new_pbhouse(s))
            .button("Finish", |s| s.quit()),
    );
}

// --- eof ---
