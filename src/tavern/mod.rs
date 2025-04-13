use structs::list::PBHouse;

// --- my stuff ---
pub(crate) mod enums;
pub(crate) mod functions;
mod implementations;
pub(crate) mod structs;
pub(crate) mod traits;

pub fn get_pbhouse_plain_text() -> String {
    let pub_and_bed_house = PBHouse::new();
    format!("\n \n {} \n \n", pub_and_bed_house)
}
// --- eof ---
