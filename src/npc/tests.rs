// ---- start of tests ----

mod suite {
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
        debug_assert!(test_pilot.height_desc == "about average");
        debug_assert!(test_pilot.build_desc == "about average");
        debug_assert!(test_pilot.hair_color == HairColorCode::Blonde);

    }
} // mod tests
// ---- end of file ----
