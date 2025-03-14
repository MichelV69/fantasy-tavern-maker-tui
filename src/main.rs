#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use cursive;
use cursive::view::Scrollable;
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
    app.version_build = 39;
    app.version_major = 0;
    app.version_minor = 6;
    app.version_fix = 13;

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
    for line in &pbh.general_info() {
        player_text += &line;
    }

    for line in &pbh.establishment_history_notes {
        gm_text += &line;
    }

    gm_text += "\n\n";

    for line in &pbh.redlight_services {
        gm_text += &line;
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
                            .fixed_width(42)
                            .scrollable()
                            .scroll_y(true),
                    ),
            )
            .button("Quit", |s| s.quit())
            .button("Another", |s| get_new_pbhouse(s))
            .h_align(cursive::align::HAlign::Center),
    );
}

// --- eof ---
