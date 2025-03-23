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

    use crate::{dice_bag::Tower::{self, DiceResult, RollDice}, structs::List::App};
    use rand::prelude::*;
    use super::lib::fnset::{read_psv_file, RollTable};


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
        pub quirk_emotional: String,
        pub quirk_physical: String,
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
    /// debug_assert!(new_npc.npc_type == NpcTypeCode::Patron);
    /// debug_assert!(new_npc.gender == GenderCode::Androgynous);
    /// debug_assert!(new_npc.public_name == "New NPC");
    /// debug_assert!(new_npc.task_description == "Realm's Most Interesting Person");
    /// debug_assert!(new_npc.species == SpeciesCode::Dragonborn);
    /// debug_assert!(new_npc.height_desc == "about average");
    /// debug_assert!(new_npc.build_desc == "about average");
    /// debug_assert!(new_npc.hair_color == HairColorCode::Blonde);
    /// debug_assert!(new_npc.hair_style == HairStyleCode::BunchOfBeadedBraids);
    /// debug_assert_ne!(new_npc.task_description, "Realm's Most Interesting Person");;
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
                hair_style: HairStyleCode::BunchOfBeadedBraids,
                eye_color: EyeColorCode::Amber,
                quirk_emotional: "nothing interesting".into(),
                quirk_physical: "nothing interesting".into(),
                notable_attribute_positive: "nothing interesting".into(),
                notable_attribute_negative: "nothing interesting".into(),
                schtick_ability_description: "nothing interesting".into(),
            }
        }

        pub fn set_random_npc_type_code(&mut self) -> () {
            self.npc_type = rand::random();
        }

        pub fn set_random_gender(&mut self) -> () {
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

        pub fn set_random_task_description(&mut self, app: App) -> () {
            let test_file = "table-RandomTaskDesc.psv";
            let psv_file_contents = read_psv_file(test_file, &app);

            let mut task_table:  Vec<RollTable> = Vec::with_capacity(42);
            let ptr: usize = 0;
            let mut high: i8 = 0;
            for line in psv_file_contents {

                let mut low :i8 = 0;
                if ptr > 0 {
                    low = task_table[ptr -1].high+1;
                }

                high = low + line.0;
                let result = line.1;

                let to_push = RollTable{low: low, high: high, result};
                task_table.push(to_push);
            }

            // roll from the table
            let mut rng = rand::rng();
            let table_roll = rng.random_range(0..=high);
            let mut table_result : String = "".into();
            for line in task_table{
                if line.low >= table_roll
                && line.high <= table_roll {table_result = line.result};
            }
            self.task_description = table_result;
            return ;
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
    #[derive(PartialEq)]
    pub enum EyeColorCode {
        Amber,
        Blue,
        Brown,
        Green,
        Grey,
        Hazel,
        Red,
    }

    #[derive(PartialEq)]
    pub enum HairStyleCode {
        BunchOfBeadedBraids,
        CrewCut,
        CroppedMohawk,
        FewShortBraids,
        LongAndLoose,
        LongBraid,
        LongCurls,
        LongMohawk,
        LongPonytail,
        ShavedClean,
        ShortAndLoose,
        ShortBraid,
        ShortCurls,
        ShortMohawk,
        ShortPonytail,
        TopKnot,
        TrioOfLongBraids,
    }

    #[derive(PartialEq)]
    pub enum HairColorCode {
        Blonde,
        Blue,
        Brown,
        Dark,
        Green,
        Red,
        SilverGrey,
        White,
    }

    #[derive(PartialEq)]
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
} //pub mod NPC
mod lib;
#[cfg(test)]
mod tests;
