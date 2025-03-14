#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use cursive;
use cursive::Cursive;
use cursive::view::Resizable;
use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::TextView;
use tracing::info;

// --- my stuff ---
mod dice_bag;
mod tavern;

use crate::dice_bag::*;
use crate::tavern::structs::List::App;
use crate::tavern::*;
use tavern::structs::List::PBHouse;

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
    let pbh = PBHouse::new();
    let mut gm_text: String = "".into();
    let mut player_text: String = "".into();

    //---
    let dialog_title = format!("P&B House: the {}", &pbh.name);
    /*
        pub struct PBHouse {
        pub name: String,
        pub mood: String,
        pub lighting: String,
        pub smells: String,
        pub size: PBHouseSize,
        pub posted_sign: String,
        pub house_drink: HouseDrink,
        pub house_dish: HouseDish,
        pub establishment_quality: EstablishmentQuality,
        pub establishment_history_notes: Vec<String>,
        pub redlight_services: Vec<String>,
    }
     */

    for line in &pbh.general_info() {
        player_text += &line;
    }

    //---
    s.pop_layer();
    s.add_layer(
        Dialog::new()
            .title(dialog_title)
            .content(
                LinearLayout::horizontal()
                    .child(Dialog::text(gm_text).title("GM Notes").fixed_width(42))
                    .child(
                        Dialog::text(player_text)
                            .title("Player Notes")
                            .fixed_width(42),
                    ),
            )
            .button("Quit", |s| s.quit())
            .h_align(cursive::align::HAlign::Center),
    );
}

// .button("Another", |s| get_new_pbhouse(s))
// .button("Finish", |s| s.quit()),

// --- eof ---
