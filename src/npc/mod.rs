// ---- start of file ----
/// Provides structs and functions required to generate simple
/// fantasy Dnd5e Non-Player Characters
pub mod Build {

    use std::default;

    use rand::{
        Rng,
        distr::{Distribution, StandardUniform},
    };
    use tracing::{Level, event};

    use super::lib::fnset::{RollTable, read_psv_file};
    use crate::{
        dice_bag::Tower::{self, DiceResult, RollDice},
        tavern::structs::List::App,
    };
    use rand::prelude;

    pub struct Profile {
        pub npc_type: NpcTypeCode,
        pub gender: GenderCode,
        pub public_name: String,
        pub task_description: String,
        pub species: SpeciesCode,
        pub height_desc: String,
        pub build_desc: String,
        pub hair_color: HairColorCode,
        pub hair_style: HairStyleCode,
        pub eye_color: EyeColorCode,
        pub quirk_emotional: QuirkEmotional,
        pub quirk_physical: QuirkPhysical,
        pub notable_attribute_positive: String,
        pub notable_attribute_negative: String,
        pub schtick_ability_description: String,
    }

    ///
    ///  used to create a new NPC profile for use with Dnd5e
    ///
    ///  # example
    ///
    /// ```
    /// let new_npc: Profile = Profile::new();
    ///
    ///  debug_assert_eq!(new_npc.npc_type, NpcTypeCode::Patron);
    ///  debug_assert_eq!(new_npc.gender, GenderCode::Androgynous);
    ///  debug_assert_eq!(new_npc.public_name, "New NPC");
    ///  debug_assert_eq!(new_npc.task_description, "Realm's Most Interesting Person");
    ///  debug_assert_eq!(new_npc.species, SpeciesCode::Dragonborn);
    ///  debug_assert_eq!(new_npc.height_desc, "about average");
    ///  debug_assert_eq!(new_npc.build_desc, "about average");
    ///  debug_assert_eq!(new_npc.hair_color, HairColorCode::Blonde);
    ///  debug_assert_eq!(new_npc.hair_style, HairStyleCode::BeadedBraided);
    ///  debug_assert_eq!(new_npc.eye_color, EyeColorCode::Red);
    ///  debug_assert_eq!(new_npc.quirk_emotional, QuirkEmotional::Manic);
    ///  debug_assert_eq!(new_npc.quirk_physical,  QuirkPhysical::SubstantialWineStain);
    /// ```
    impl Profile {
        pub fn new() -> Self {
            Profile {
                npc_type: NpcTypeCode::Patron,
                gender: GenderCode::Androgynous,
                public_name: "New NPC".into(),
                task_description: "Realm's Most Interesting Person".into(),
                species: SpeciesCode::Dragonborn,
                height_desc: "about average".into(),
                build_desc: "about average".into(),
                hair_color: HairColorCode::Blonde,
                hair_style: HairStyleCode::BeadedBraided,
                eye_color: EyeColorCode::Red,
                quirk_emotional: QuirkEmotional::Manic,
                quirk_physical: QuirkPhysical::SubstantialWineStain,
                notable_attribute_positive: "nothing interesting".into(),
                notable_attribute_negative: "nothing interesting".into(),
                schtick_ability_description: "nothing interesting".into(),
            }
        }

        pub fn random_appearance(&mut self, app: &App) {
            self.set_random_gender();
            self.set_random_species(&app);
            self.set_random_height_desc();
            self.set_random_build_desc();
            self.set_random_hair_color(&app);
            self.set_random_hair_style(&app);
            self.set_random_eye_color(&app);
            self.set_random_quirk_physical(&app);
        }

        pub fn set_random_quirk_physical(&mut self, app: &App) {
            let test_file = "table-RandomQuirkPhysical.psv";
            let psv_file_contents = read_psv_file(test_file, &app);
            let result = Self::roll_from_table(psv_file_contents);
            println!("set_random_quirk_physical:result:[{}]", result);
            self.quirk_physical = match result {
                val if val == "None" => QuirkPhysical::None,
                val if val == "SlightScar" => QuirkPhysical::SlightScar,
                val if val == "NoticeableScar" => QuirkPhysical::NoticeableScar,
                val if val == "SubstantialScar" => QuirkPhysical::SubstantialScar,
                val if val == "InfrequentSquint" => QuirkPhysical::InfrequentSquint,
                val if val == "FrequentSquint" => QuirkPhysical::FrequentSquint,
                val if val == "ConstantSquint" => QuirkPhysical::ConstantSquint,
                val if val == "SmallTattoo" => QuirkPhysical::SmallTattoo,
                val if val == "MinorScarification" => QuirkPhysical::MinorScarification,
                val if val == "NoticeableTattoo" => QuirkPhysical::NoticeableTattoo,
                val if val == "NoticeableScarification" => QuirkPhysical::NoticeableScarification,
                val if val == "SubstantialTattoo" => QuirkPhysical::SubstantialTattoo,
                val if val == "SubstantialScarification" => QuirkPhysical::SubstantialScarification,
                val if val == "SlightWineStain" => QuirkPhysical::SlightWineStain,
                val if val == "NoticeableWineStain" => QuirkPhysical::NoticeableWineStain,
                val if val == "SubstantialWineStain" => QuirkPhysical::SubstantialWineStain,
                _ => panic!("set_random_quirk_physical result: [{result}]"),
            };
        }

        pub fn set_random_quirk_emotional(&mut self, app: &App) {
            let test_file = "table-RandomQuirkEmotional.psv";
            let psv_file_contents = read_psv_file(test_file, &app);
            let result = Self::roll_from_table(psv_file_contents);
            println!("set_random_quirk_emotional:result:[{}]", result);
            self.quirk_emotional = match result {
                val if val == "None" => QuirkEmotional::None,
                val if val == "DistrustfulOfAdventurers" => {
                    QuirkEmotional::DistrustfulOfAdventurers
                }
                val if val == "CheerfulToAdventurers" => QuirkEmotional::CheerfulToAdventurers,
                val if val == "Shy" => QuirkEmotional::Shy,
                val if val == "Grumpy" => QuirkEmotional::Grumpy,
                val if val == "Playful" => QuirkEmotional::Playful,
                val if val == "Loud" => QuirkEmotional::Loud,
                val if val == "Miserly" => QuirkEmotional::Miserly,
                val if val == "Hyperfocused" => QuirkEmotional::Hyperfocused,
                val if val == "Belligerent" => QuirkEmotional::Belligerent,
                val if val == "PhysicallyAffectionate" => QuirkEmotional::PhysicallyAffectionate,
                val if val == "Generous" => QuirkEmotional::Generous,
                val if val == "EasilyDistracted" => QuirkEmotional::EasilyDistracted,
                val if val == "Depressive" => QuirkEmotional::Depressive,
                val if val == "Manic" => QuirkEmotional::Manic,
                _ => panic!("set_random_quirk_emotional result: [{result}]"),
            };
        }

        pub fn set_random_eye_color(&mut self, app: &App) {
            let test_file = "table-RandomEyeColor.psv";
            let psv_file_contents = read_psv_file(test_file, &app);
            let result = Self::roll_from_table(psv_file_contents);
            println!("set_random_eye_color:result:[{}]", result);
            self.eye_color = match result {
                val if val == "Brown" => EyeColorCode::Brown,
                val if val == "Hazel" => EyeColorCode::Hazel,
                val if val == "Blue" => EyeColorCode::Blue,
                val if val == "Green" => EyeColorCode::Green,
                val if val == "Grey" => EyeColorCode::Grey,
                val if val == "Amber" => EyeColorCode::Amber,
                val if val == "Purple" => EyeColorCode::Purple,
                val if val == "Red" => EyeColorCode::Red,
                _ => panic!("set_random_eye_color result: [{result}]"),
            };
        }

        pub fn set_random_hair_style(&mut self, app: &App) {
            let test_file = "table-RandomHairStyle.psv";
            let psv_file_contents = read_psv_file(test_file, &app);
            let result = Self::roll_from_table(psv_file_contents);
            println!("set_random_hair_color:result:[{}]", result);
            self.hair_style = match result {
                val if val == "ShortBraided" => HairStyleCode::ShortBraided,
                val if val == "ShortPonytail" => HairStyleCode::ShortPonytail,
                val if val == "ShortLoose" => HairStyleCode::ShortLoose,
                val if val == "ShortCurls" => HairStyleCode::ShortCurls,
                val if val == "LongPonytail" => HairStyleCode::LongPonytail,
                val if val == "ShortMohawk" => HairStyleCode::ShortMohawk,
                val if val == "LongLoose" => HairStyleCode::LongLoose,
                val if val == "LongCurls" => HairStyleCode::LongCurls,
                val if val == "LongBraided" => HairStyleCode::LongBraided,
                val if val == "TopKnot" => HairStyleCode::TopKnot,
                val if val == "ShavedClean" => HairStyleCode::ShavedClean,
                val if val == "TallMohawk" => HairStyleCode::TallMohawk,
                val if val == "CroppedMohawk" => HairStyleCode::CroppedMohawk,
                val if val == "CrewCut" => HairStyleCode::CrewCut,
                val if val == "BeadedBraided" => HairStyleCode::BeadedBraided,

                _ => panic!("set_random_hair_color result: [{result}]"),
            };
        }

        pub fn set_random_hair_color(&mut self, app: &App) {
            let test_file = "table-RandomHairColor.psv";
            let psv_file_contents = read_psv_file(test_file, &app);
            let result = Self::roll_from_table(psv_file_contents);
            println!("set_random_hair_color:result:[{}]", result);
            self.hair_color = match result {
                val if val == "Dark" => HairColorCode::Brown,
                val if val == "Brown" => HairColorCode::Brown,
                val if val == "Blonde" => HairColorCode::Blonde,
                val if val == "White" => HairColorCode::White,
                val if val == "Silver-Grey" => HairColorCode::SilverGrey,
                val if val == "Red" => HairColorCode::Red,
                val if val == "Green" => HairColorCode::Red,
                val if val == "Blue" => HairColorCode::Blue,
                val if val == "Purple" => HairColorCode::Purple,
                _ => panic!("set_random_hair_color result: [{result}]"),
            };
        }

        pub fn set_random_build_desc(&mut self) {
            let roll_2d6: i16 = <Tower::DiceResult>::from_string("2d6").get_total();
            self.build_desc = match roll_2d6 {
                2 => <Tower::DiceResult>::inline_replace("gaunt (-[3d8+6]%)"),
                3 => <Tower::DiceResult>::inline_replace("lean (-[2d8+3]%)"),
                4..=5 => <Tower::DiceResult>::inline_replace("slightly angular (-[1d8+1]%)"),
                6..=8 => <Tower::DiceResult>::inline_replace("medium build ([2d4-4]%)"),
                9..=10 => <Tower::DiceResult>::inline_replace("slightly husky (+[1d8+1]%)"),
                11 => <Tower::DiceResult>::inline_replace("stout (+[2d8+3]%)"),
                12 => <Tower::DiceResult>::inline_replace("portly (+[3d8+6]%)"),
                _ => panic!("Rolled weird on 2d6 : [{roll_2d6}] "),
            };
        }

        pub fn set_random_height_desc(&mut self) {
            let roll_2d6: i16 = <Tower::DiceResult>::from_string("2d6").get_total();
            self.height_desc = match roll_2d6 {
                2 => <Tower::DiceResult>::inline_replace("very short (-[3d8+6]%)"),
                3 => <Tower::DiceResult>::inline_replace("short (-[2d8+3]%)"),
                4..=5 => <Tower::DiceResult>::inline_replace("short-ish (-[1d8+1]%)"),
                6..=8 => <Tower::DiceResult>::inline_replace("average height ([2d4-4]%)"),
                9..=10 => <Tower::DiceResult>::inline_replace("tall-ish (+[1d8+1]%)"),
                11 => <Tower::DiceResult>::inline_replace("tall (+[2d8+3]%)"),
                12 => <Tower::DiceResult>::inline_replace("very tall (+[3d8+6]%)"),
                _ => panic!("Rolled weird on 2d6 : [{roll_2d6}] "),
            };
        }

        pub fn set_random_species(&mut self, app: &App) {
            let test_file = "table-RandomSpeciesByWeight.psv";
            let psv_file_contents = read_psv_file(test_file, &app);
            let result = Self::roll_from_table(psv_file_contents);
            println!("set_random_species:result:[{}]", result);
            self.species = match result {
                val if val == "human" => SpeciesCode::Human,
                val if val == "dwarf" => SpeciesCode::Dwarf,
                val if val == "halfling" => SpeciesCode::Halfling,
                val if val == "elf" => SpeciesCode::Elf,
                val if val == "gnome" => SpeciesCode::Gnome,
                val if val == "tiefling" => SpeciesCode::Tiefling,
                val if val == "dragonborn" => SpeciesCode::Dragonborn,
                _ => panic!("set_random_species result: [{result}]"),
            };
        }

        fn roll_from_table(psv_file_contents: Vec<(i16, String)>) -> String {
            // build the table
            let mut result_table: Vec<RollTable> = Vec::with_capacity(42);

            let mut ptr: usize = 0;
            let mut high: i16 = 0;
            for line in psv_file_contents {
                let mut low: i16 = 1;
                if result_table.len() > 1 {
                    low = result_table[result_table.len() - 1].high + 1;
                }
                high = low + line.0 - 1;
                let result = line.1;

                let to_push: RollTable = RollTable { low, high, result };
                println!("roll_from_table::to_push:[{:#?}]", to_push);
                result_table.push(to_push);
            }

            // roll from the table
            println!("roll_from_table::high:[{}]", high);
            let mut rng = rand::rng();
            let table_roll = rng.random_range(1..=high);
            let mut table_result: String = "".into();
            for line in result_table {
                if (table_roll >= line.low) && (table_roll <= line.high) {
                    table_result = line.result
                };
            }

            event!(Level::INFO, "table_result[{:#?}]", table_result);
            println!(
                "roll_from_table::table_roll:result:[{}:{}]",
                table_roll, table_result
            );
            if table_result.is_empty() {
                panic!("table_result should never be empty! [{table_roll}:{high}]")
            }

            table_result
        }

        pub fn set_random_npc_type_code(&mut self) {
            self.npc_type = rand::random();
        }

        pub fn set_random_gender(&mut self) {
            let set_of_rolls = Tower::DiceResult::from_string("1d10");
            let total_of_rolls = set_of_rolls.get_total();

            self.gender = match total_of_rolls {
                1..=5 => GenderCode::Male,
                6..=8 => GenderCode::Female,
                9..=10 => GenderCode::Androgynous,
                default => panic!(
                    "Unexpected roll out of bounds {}:{}",
                    set_of_rolls.get_request(),
                    total_of_rolls
                ),
            }
        }

        pub fn set_random_task_description(&mut self, app: &App) {
            let test_file = "table-RandomTaskDesc.psv";
            let psv_file_contents = read_psv_file(test_file, &app);
            self.task_description = Self::roll_from_table(psv_file_contents);
        }
    }

    // ---
    impl Distribution<NpcTypeCode> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NpcTypeCode {
            let index: u8 = rng.random_range(0..=2);
            match index {
                0 => NpcTypeCode::Patron,
                1 => NpcTypeCode::Staff,
                2 => NpcTypeCode::StoryCharacter,
                _ => unreachable!(),
            }
        }
    }

    // ---
    #[derive(PartialEq, Debug)]
    pub enum EyeColorCode {
        Brown,
        Hazel,
        Blue,
        Green,
        Grey,
        Amber,
        Purple,
        Red,
    }

    #[derive(PartialEq, Debug)]
    pub enum HairStyleCode {
        BeadedBraided,
        CrewCut,
        CroppedMohawk,
        LongLoose,
        LongBraided,
        LongCurls,
        TallMohawk,
        LongPonytail,
        ShavedClean,
        ShortLoose,
        ShortBraided,
        ShortCurls,
        ShortMohawk,
        ShortPonytail,
        TopKnot,
    }

    #[derive(PartialEq, Debug)]
    pub enum HairColorCode {
        Blonde,
        Blue,
        Brown,
        Dark,
        Green,
        Purple,
        Red,
        SilverGrey,
        White,
    }

    #[derive(PartialEq, Debug)]
    pub enum SpeciesCode {
        Dragonborn,
        Dwarf,
        Elf,
        Gnome,
        Halfling,
        Human,
        Tiefling,
    }

    #[derive(PartialEq, Debug)]
    pub enum NpcTypeCode {
        Patron,
        Staff,
        StoryCharacter,
    }

    #[derive(PartialEq, Debug)]
    pub enum GenderCode {
        Androgynous,
        Female,
        Male,
    }

    #[derive(PartialEq, Debug)]
    pub enum QuirkEmotional {
        Belligerent,
        CheerfulToAdventurers,
        Depressive,
        DistrustfulOfAdventurers,
        EasilyDistracted,
        Generous,
        Grumpy,
        Hyperfocused,
        Loud,
        Manic,
        Miserly,
        None,
        PhysicallyAffectionate,
        Playful,
        Shy,
    }

    #[derive(PartialEq, Debug)]
    pub enum QuirkPhysical {
        None,
        SlightScar,
        NoticeableScar,
        SubstantialScar,
        InfrequentSquint,
        FrequentSquint,
        ConstantSquint,
        SmallTattoo,
        MinorScarification,
        NoticeableTattoo,
        NoticeableScarification,
        SubstantialTattoo,
        SubstantialScarification,
        SlightWineStain,
        NoticeableWineStain,
        SubstantialWineStain,
    }
} //pub mod NPC
mod lib;
#[cfg(test)]
mod tests;
