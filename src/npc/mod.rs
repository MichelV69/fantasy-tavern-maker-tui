// ---- start of file ----
pub mod Build {
    use tracing::{Level, event};

    pub struct Profile {
        pub npc_type: NpcTypeCode,
        pub gender: GenderCode,
        pub public_name: String,
        pub task_description: String,
        pub species: SpeciesCode,
        pub height_desc: String,
    }

    impl Profile {
        pub fn new() -> Self {
            Profile {
                npc_type: NpcTypeCode::Patron,
                gender: GenderCode::Androgynous,
                public_name: "New NPC".into(),
                task_description: "Realm's Most Interesting Person".into(),
                species: SpeciesCode::Dragonborn,
                height_desc: "about average".into()
            }
        }
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

// ---- start of tests ----
#[cfg(test)]
mod tests {
    use super::*;
    use crate::npc::Build::*;
    use tracing::{Level, event};

    #[test]
    fn profile_new() {
        let test_pilot: Profile = Profile::new();

        debug_assert!(test_pilot.npc_type == NpcTypeCode::Patron);
        debug_assert!(test_pilot.gender == GenderCode::Androgynous);
        debug_assert!(test_pilot.public_name == "New NPC");
        debug_assert!(test_pilot.task_description == "Realm's Most Interesting Person");
        debug_assert!(test_pilot.species == SpeciesCode::Dragonborn);
        debug_assert!(test_pilot.height_desc  == "about average");

    }
} // mod tests
// ---- end of file ----
