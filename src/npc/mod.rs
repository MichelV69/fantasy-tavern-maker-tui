// ---- start of file ----
/// Provides structs and functions required to generate simple
/// fantasy Dnd5e Non-Player Characters
pub(crate) mod impls;
pub mod build {
    use strum::Display;

    use crate::narrative_time_manager::ntm::TimeSlot;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Profile {
        pub npc_type: NpcTypeCode,
        pub gender: GenderCode,
        pub task_description: String,
        pub species: SpeciesCode,
        pub height_desc: String,
        pub build_desc: String,
        pub hair_color: HairColorCode,
        pub hair_style: HairStyleCode,
        pub eye_color: EyeColorCode,
        pub quirk_emotional: QuirkEmotional,
        pub quirk_physical: QuirkPhysical,
        pub notable_attribute_positive: Attribute,
        pub notable_attribute_negative: Attribute,
        pub encounter_slots: Vec<TimeSlot>,
        // pub schtick_ability_description: String,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Attribute {
        pub description: String,
        pub modifier: i8,
    }

    ///
    ///  used to create a new NPC profile for use with Dnd5e
    ///
    ///  # example
    ///
    /// ```
    /// let new_npc: Profile = Profile::new();
    ///
    ///  assert_eq!(new_npc.npc_type, NpcTypeCode::Patron);
    ///  assert_eq!(new_npc.gender, GenderCode::Androgynous);
    ///  assert_eq!(new_npc.public_name, "New NPC");
    ///  assert_eq!(new_npc.task_description, "Realm's Most Interesting Person");
    ///  assert_eq!(new_npc.species, SpeciesCode::Dragonborn);
    ///  assert_eq!(new_npc.height_desc, "about average");
    ///  assert_eq!(new_npc.build_desc, "about average");
    ///  assert_eq!(new_npc.hair_color, HairColorCode::Blonde);
    ///  assert_eq!(new_npc.hair_style, HairStyleCode::BeadedBraided);
    ///  assert_eq!(new_npc.eye_color, EyeColorCode::Red);
    ///  assert_eq!(new_npc.quirk_emotional, QuirkEmotional::Manic);
    ///  assert_eq!(new_npc.quirk_physical,  QuirkPhysical::SubstantialWineStain);
    /// ```

    // ---
    #[derive(PartialEq, Debug, Clone, Copy)]
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

    #[derive(PartialEq, Debug, Clone, Copy)]
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

    #[derive(PartialEq, Debug, Clone)]
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

    #[derive(PartialEq, Debug, Display, Clone, Copy)]
    pub enum SpeciesCode {
        Dragonborn,
        Dwarf,
        Elf,
        Gnome,
        Halfling,
        Human,
        Tiefling,
    }

    #[derive(PartialEq, Debug, Display, Clone, Copy)]
    pub enum NpcTypeCode {
        Patron,
        Staff,
        StoryCharacter,
    }

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum GenderCode {
        Androgynous,
        Female,
        Male,
    }

    #[derive(PartialEq, Debug, Clone, Copy)]
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

    #[derive(PartialEq, Debug, Clone, Copy)]
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
