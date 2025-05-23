// ---- start of file ----

pub mod list {

    use crate::tavern::enums::list::EstablishmentQualityLevel;
    use crate::tavern::enums::list::SizeList;
    use crate::tavern::enums::list::{BedTypeList, RSLCode};

    #[derive(Debug, Clone)]
    pub struct App {
        pub name: String,
        pub version_major: i16,
        pub version_minor: i16,
        pub version_fix: i16,
        pub version_build: i16,
    }

    #[derive(Debug, Clone)]
    pub struct RedlightService {
        pub service: RSLCode,
        pub dc: i16,
    }

    #[derive(Debug, Clone)]
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
        pub redlight_services: Vec<RedlightService>,
    }

    // ---
    #[derive(Debug, Clone)]
    pub struct EstablishmentQuality {
        pub level: EstablishmentQualityLevel,
        pub rooms: String,
        pub meals: String,
    }

    #[derive(Debug, Clone)]
    pub struct PBHouseSize {
        pub size_description: SizeList,
        pub table_count: i16,
        pub common_bed_type: BedTypeList,
        pub common_bed_count: i16,
        pub private_room_count: i16,
    }

    #[derive(Debug, Clone)]
    pub struct HouseDrink {
        pub desc: String,
        pub price: String,
    }

    #[derive(Debug, Clone)]
    pub struct HouseDish {
        pub desc: String,
        pub price: String,
    }
}
// ---- end of file ----
