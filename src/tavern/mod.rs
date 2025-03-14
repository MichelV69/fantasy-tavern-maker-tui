#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use tracing::info;

// --- my stuff ---
mod enums;
use enums::*;

mod functions;
use functions::*;

mod implementations;
use implementations::*;

pub(crate) mod structs;
use structs::{List::PBHouse, *};

pub(crate) mod traits;
use traits::*;

pub fn get_pbhouse_plain_text() -> String {
    let pub_and_bed_house = PBHouse::new();
    return format!("\n \n {} \n \n", pub_and_bed_house);
}
// --- eof ---
