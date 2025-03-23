// ---- start of tests ----

mod suite {
    use std::path::Path;

    use super::*;
    use crate::{
        dice_bag::Tower::{self, RollDice},
        npc::{lib::fnset::read_psv_file, Build::*}, structs::List::App,
    };
    use tracing::{Level, event};

    #[test]
    fn profile_new() {
        let new_npc: Profile = Profile::new();

        debug_assert!(new_npc.npc_type == NpcTypeCode::Patron);
        debug_assert!(new_npc.gender == GenderCode::Androgynous);
        debug_assert!(new_npc.public_name == "New NPC");
        debug_assert!(new_npc.task_description == "Realm's Most Interesting Person");
        debug_assert!(new_npc.species == SpeciesCode::Dragonborn);
        debug_assert!(new_npc.height_desc == "about average");
        debug_assert!(new_npc.build_desc == "about average");
        debug_assert!(new_npc.hair_color == HairColorCode::Blonde);
        debug_assert!(new_npc.hair_style == HairStyleCode::BunchOfBeadedBraids);
        debug_assert!(new_npc.eye_color == EyeColorCode::Amber);
        debug_assert!(new_npc.quirk_emotional == "nothing interesting");
        debug_assert!(new_npc.quirk_physical == "nothing interesting");
        debug_assert!(new_npc.notable_attribute_positive == "nothing interesting");
        debug_assert!(new_npc.notable_attribute_negative == "nothing interesting");
        debug_assert!(new_npc.schtick_ability_description == "nothing interesting");
    }

    #[test]
    fn set_random_npc_type_code() {
        let mut new_npc: Profile = Profile::new();

        while new_npc.npc_type == NpcTypeCode::Patron {
            new_npc.set_random_npc_type_code();
        }

        event!(Level::INFO, "new_npc.npc_type[{:#?}]", new_npc.npc_type);
        println!("new_npc.npc_type[{:#?}]", new_npc.npc_type);
        debug_assert_ne!(new_npc.npc_type, NpcTypeCode::Patron);
    }

    #[test]
    fn set_random_gender() {
        let mut new_npc: Profile = Profile::new();

        while new_npc.gender == GenderCode::Androgynous {
            new_npc.set_random_gender();
        }

        event!(Level::INFO, "new_npc.gender[{:#?}]", new_npc.gender);
        println!("new_npc.gender[{:#?}]", new_npc.gender);
        debug_assert_ne!(new_npc.gender, GenderCode::Androgynous);
    }

    #[test]
    fn set_random_task_description() {
        let mut app: App = App::new();
        app.name = "fantasy-tavern-maker-tui".into();

        let mut new_npc: Profile = Profile::new();
        new_npc.set_random_task_description(app);

        event!(
            Level::INFO,
            "new_npc.task_description[{:#?}]",
            new_npc.task_description
        );
        println!("new_npc.task_description[{:#?}]", new_npc.task_description);
        debug_assert_ne!(new_npc.task_description, "Realm's Most Interesting Person");
    }

    #[test]
    fn test_read_psv_file() {

        let mut app: App = App::new();
        app.name = "fantasy-tavern-maker-tui".into();

        let test_file = "table-RandomTaskDesc.psv";
        let psv_file_contents = read_psv_file(test_file, &app);

        event!(Level::INFO, "psv_file_contents[0].0[{:#?}]", psv_file_contents[0].0);
        println!("psv_file_contents[0].0[{:#?}]",psv_file_contents[0].0);

        debug_assert_eq!(psv_file_contents[0].0, 40);
        debug_assert_eq!(psv_file_contents[0].1, "Commonfolk");

    }
} // mod tests
// ---- end of file ----
