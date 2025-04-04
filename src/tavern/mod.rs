#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use structs::List::PBHouse;
use tracing::info;

// --- my stuff ---
mod enums;
mod functions;
mod implementations;
pub(crate) mod structs;
pub(crate) mod traits;

pub fn get_pbhouse_plain_text() -> String {
    let pub_and_bed_house = PBHouse::new();
    format!("\n \n {} \n \n", pub_and_bed_house)
}
// --- eof ---
