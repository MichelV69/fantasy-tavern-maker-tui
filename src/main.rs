#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use tracing::info;

// --- my stuff ---
mod dice_bag;
use crate::dice_bag::*;
mod tavern;
use crate::tavern::*;

// --- local cli code
fn app() -> String {
    let pub_and_bed_house = PBHouse::new();
    format!("\n \n {} \n \n", pub_and_bed_house)
}
// --- eof ---
