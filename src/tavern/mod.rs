#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use tracing::info;

// --- my stuff ---
mod traits;
use tavern::traits::List::*;

mod implementations;
use crate::implementations::List::*;

mod functions;
use functions::*;

mod enums;
use crate::enums::List::*;

mod structs;
use crate::structs::List::*;

// --- eof ---
