#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use cursive::views::Dialog;
use cursive::views::TextView;
use cursive::Cursive;
use tracing::info;
use cursive;

// --- my stuff ---
mod dice_bag;
mod tavern;

use crate::dice_bag::*;
use crate::tavern::*;

// todo!("add types of mead to the drink list");

// --- local cli code
fn main() -> () {
  let mut siv = cursive::default();

  siv.add_layer(Dialog::text("Welcome to fantasy-tavern-maker-tui")
      .title("fantasy-tavern-maker-tui")
      .button("New",|s| get_new_pbhouse(s))
      .button("Finish", |s| s.quit()));

 siv.run()
}

fn get_new_pbhouse(s: &mut Cursive) -> () {
    s.pop_layer();
    s.add_layer(Dialog::text(get_pbhouse())
        .title("Results")
        .button("Another",|s| get_new_pbhouse(s))
        .button("Finish", |s| s.quit()));
}

// --- eof ---
