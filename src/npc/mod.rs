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
    }

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
                hair_color: HairColorCode::Blonde
            }
        }
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
