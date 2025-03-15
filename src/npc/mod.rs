// ---- start of file ----
/// Provides structs and functions required to generate simple
/// fantasy Dnd5e Non-Player Characters
pub mod Build {
    use tracing::{Level, event};

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
            }
        }
    }

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

    #[derive(PartialEq)]
    pub enum NpcTypeCode {
        Patron,
        Staff,
        StoryCharacter,
    }

    #[derive(PartialEq)]
    pub enum GenderCode {
        Androgynous,
        Female,
        Male,
    }
} //pub mod NPC
#[cfg(test)]
mod tests;
