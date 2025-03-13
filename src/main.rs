#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use tracing::info;

// --- my stuff ---
mod dice_bag;
mod tavern;

use crate::dice_bag::*;
use crate::tavern::*;

// todo!("add types of mead to the drink list");

// --- local cli code
fn main() -> () {
    let pub_and_bed_house = PBHouse::new();
    format!("\n \n {} \n \n", pub_and_bed_house)
}
// --- eof ---
