// ---- start of tests ----

mod suite {
    use std::path::Path;

    use super::*;
    use crate::{
        dice_bag::Tower::{self, RollDice},
        npc::{
            Build::{
                self, Attribute, EyeColorCode, GenderCode, HairColorCode, HairStyleCode,
                NpcTypeCode, Profile, QuirkEmotional, QuirkPhysical, SpeciesCode,
            },
            lib::fnset::read_psv_file,
        },
        tavern::structs::List::App,
    };
    use tracing::{Level, event};

    #[test]
    fn profile_new() {
        let new_npc: Profile = Profile::new();
        let default_attr = Attribute {
            description: "nothing interesting".into(),
            modifier: -13,
        };

        debug_assert_eq!(new_npc.npc_type, NpcTypeCode::Patron);
        debug_assert_eq!(new_npc.gender, GenderCode::Androgynous);
        debug_assert_eq!(new_npc.public_name, "New NPC");
        debug_assert_eq!(new_npc.task_description, "Realm's Most Interesting Person");
        debug_assert_eq!(new_npc.species, SpeciesCode::Dragonborn);
        debug_assert_eq!(new_npc.height_desc, "about average");
        debug_assert_eq!(new_npc.build_desc, "about average");
        debug_assert_eq!(new_npc.hair_color, HairColorCode::Blonde);
        debug_assert_eq!(new_npc.hair_style, HairStyleCode::BeadedBraided);
        debug_assert_eq!(new_npc.eye_color, EyeColorCode::Red);
        debug_assert_eq!(new_npc.quirk_emotional, QuirkEmotional::Manic);
        debug_assert_eq!(new_npc.quirk_physical, QuirkPhysical::SubstantialWineStain);
        debug_assert_eq!(new_npc.notable_attribute_positive, default_attr);
        debug_assert_eq!(new_npc.notable_attribute_negative, default_attr);
        debug_assert_eq!(new_npc.schtick_ability_description, "nothing interesting");
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
        new_npc.set_random_task_description(&app);

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

        event!(
            Level::INFO,
            "psv_file_contents[0].0[{:#?}]",
            psv_file_contents[0].0
        );
        println!("psv_file_contents[0].0[{:#?}]", psv_file_contents[0].0);

        debug_assert_eq!(psv_file_contents[0].0, 40);
        debug_assert_eq!(psv_file_contents[0].1, "Commonfolk");
    }

    #[test]
    fn set_random_species() {
        let mut app: App = App::new();
        app.name = "fantasy-tavern-maker-tui".into();

        let mut new_npc: Profile = Profile::new();

        let mut counter: i8 = 0;
        while (new_npc.species == SpeciesCode::Dragonborn) && counter < 7 {
            counter += 1;
            new_npc.set_random_species(&app);
        }

        event!(Level::INFO, "new_npc.species[{:#?}]", new_npc.species);
        println!("new_npc.species[{:#?}]", new_npc.species);
        debug_assert_ne!(new_npc.species, SpeciesCode::Dragonborn);
    }

    #[test]
    fn set_random_height_desc() {
        let mut new_npc: Profile = Profile::new();
        new_npc.set_random_height_desc();

        debug_assert_ne!(new_npc.height_desc, "about average");
    }

    #[test]
    fn set_random_build_desc() {
        let mut new_npc: Profile = Profile::new();
        new_npc.set_random_build_desc();

        debug_assert_ne!(new_npc.build_desc, "about average");
    }

    #[test]
    fn set_random_hair_color() {
        let mut app: App = App::new();
        app.name = "fantasy-tavern-maker-tui".into();

        let mut new_npc: Profile = Profile::new();

        let mut counter: i8 = 0;
        while (new_npc.hair_color == HairColorCode::Blonde) && counter < 7 {
            counter += 1;
            new_npc.set_random_hair_color(&app);
        }

        event!(Level::INFO, "new_npc.hair_color[{:#?}]", new_npc.hair_color);
        println!("new_npc.hair_color[{:#?}]", new_npc.hair_color);
        debug_assert_ne!(new_npc.hair_color, HairColorCode::Blonde);
    }

    #[test]
    fn set_random_hair_style() {
        let mut app: App = App::new();
        app.name = "fantasy-tavern-maker-tui".into();

        let mut new_npc: Profile = Profile::new();

        let mut counter: i8 = 0;
        while (new_npc.hair_style == HairStyleCode::BeadedBraided) && counter < 7 {
            counter += 1;
            new_npc.set_random_hair_style(&app);
        }

        event!(Level::INFO, "new_npc.hair_style[{:#?}]", new_npc.hair_style);
        println!("new_npc.hair_style[{:#?}]", new_npc.hair_style);
        debug_assert_ne!(new_npc.hair_style, HairStyleCode::BeadedBraided);
    }

    #[test]
    fn set_random_eye_color() {
        let mut app: App = App::new();
        app.name = "fantasy-tavern-maker-tui".into();

        let mut new_npc: Profile = Profile::new();

        let mut counter: i8 = 0;
        while (new_npc.eye_color == EyeColorCode::Red) && counter < 7 {
            counter += 1;
            new_npc.set_random_eye_color(&app);
        }

        event!(Level::INFO, "new_npc.eye_color[{:#?}]", new_npc.eye_color);
        println!("new_npc.eye_color[{:#?}]", new_npc.eye_color);
        debug_assert_ne!(new_npc.eye_color, EyeColorCode::Red);
    }

    #[test]
    fn set_random_quirk_emotional() {
        let mut app: App = App::new();
        app.name = "fantasy-tavern-maker-tui".into();

        let mut new_npc: Profile = Profile::new();

        let mut counter: i8 = 0;
        while (new_npc.quirk_emotional == QuirkEmotional::Manic) && counter < 7 {
            counter += 1;
            new_npc.set_random_quirk_emotional(&app);
        }

        event!(
            Level::INFO,
            "new_npc.quirk_emotional[{:#?}]",
            new_npc.quirk_emotional
        );
        println!("new_npc.quirk_emotional[{:#?}]", new_npc.quirk_emotional);
        debug_assert_ne!(new_npc.quirk_emotional, QuirkEmotional::Manic);
    }

    #[test]
    fn set_random_quirk_physical() {
        let mut app: App = App::new();
        app.name = "fantasy-tavern-maker-tui".into();

        let mut new_npc: Profile = Profile::new();

        let mut counter: i8 = 0;
        while (new_npc.quirk_physical == QuirkPhysical::SubstantialWineStain) && counter < 7 {
            counter += 1;
            new_npc.set_random_quirk_physical(&app);
        }

        event!(
            Level::INFO,
            "new_npc.quirk_physical[{:#?}]",
            new_npc.quirk_physical
        );
        println!("new_npc.quirk_physical[{:#?}]", new_npc.quirk_physical);
        debug_assert_ne!(new_npc.quirk_physical, QuirkPhysical::SubstantialWineStain);
    }

    #[test]
    fn set_notable_attribute_positive() {
        let mut app: App = App::new();
        app.name = "fantasy-tavern-maker-tui".into();
        let default_attr = Attribute {
            description: "nothing interesting".into(),
            modifier: -13,
        };

        let mut new_npc: Profile = Profile::new();
        new_npc.set_notable_attribute_positive(&app);

        event!(
            Level::INFO,
            "new_npc.notable_attribute_positive[{:#?}]",
            new_npc.notable_attribute_positive
        );
        println!(
            "new_npc.quirk_physical[{:#?}]",
            new_npc.notable_attribute_positive
        );
        debug_assert_ne!(new_npc.notable_attribute_positive, default_attr);
    }

    #[test]
    fn set_notable_attribute_negative() {
        let mut app: App = App::new();
        app.name = "fantasy-tavern-maker-tui".into();
        let default_attr = Attribute {
            description: "nothing interesting".into(),
            modifier: -13,
        };

        let mut new_npc: Profile = Profile::new();
        new_npc.set_notable_attribute_negative(&app);

        event!(
            Level::INFO,
            "new_npc.notable_attribute_negative[{:#?}]",
            new_npc.notable_attribute_negative
        );
        println!(
            "new_npc.quirk_physical[{:#?}]",
            new_npc.notable_attribute_negative
        );
        debug_assert_ne!(new_npc.notable_attribute_negative, default_attr);
    }
} // mod tests
// ---- end of file ----
