// ---- start of file ----

pub mod List {
    use rand::prelude::*;
    use rand_derive2::RandGen;
    use rocket::serde::Serialize;
    use strum_macros::Display;
    use strum_macros::EnumString;

    use crate::enums::List::*;

    #[derive(Debug)]
    pub struct App {
        pub name: String,
        pub version_major: i8,
        pub version_minor: i8,
        pub version_fix: i8,
    }

    #[derive(Debug, Serialize)]
    pub struct PBHouse{
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

    // ---
    #[derive(Debug, Clone, Serialize)]
    pub struct EstablishmentQuality {
        pub level: EstablishmentQualityLevel,
        pub rooms: String,
        pub meals: String,
    }

    #[derive(Debug, Serialize)]
    pub struct PBHouseSize {
        pub size_description: SizeList,
        pub table_count: i8,
        pub common_bed_type: BedTypeList,
        pub common_bed_count: i8,
        pub private_room_count: i8,
    }

    #[derive(Debug, Serialize)]
    pub struct HouseDrink {
        pub desc: String,
        pub price: String,
    }

    #[derive(Debug, Serialize)]
    pub struct HouseDish {
        pub desc: String,
        pub price: String,
    }
}
// ---- end of file ----
