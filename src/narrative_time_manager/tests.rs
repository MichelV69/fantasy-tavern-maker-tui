mod suite {

    use crate::{
        narrative_time_manager::ntm::{self, SlotNames, TimeSlot},
        npc::build::Profile,
    };
    use std::mem;

    #[test]
    fn number_of_timeslots() {
        let target_value: usize = 18;
        assert_eq!(target_value, mem::variant_count::<SlotNames>());
    }

    #[test]
    fn check_sunrise() {
        let haystack = ntm::load();
        let needle = SlotNames::Sunrise;
        let mut found = false;
        for variant in haystack.iter() {
            if needle == variant.name {
                found = true
            };
        }
        assert!(found);
    }

    #[test]
    fn check_noon() {
        let test_data = ntm::load();
        let test_pointer = 8;
        assert_eq!(test_data[test_pointer].name, SlotNames::Midday);
    }

    #[test]
    fn check_sunset() {
        let haystack = ntm::load();
        let needle = SlotNames::Sunset;
        let mut found = false;
        for variant in haystack.iter() {
            if needle == variant.name {
                found = true
            };
        }
        assert!(found);
    }

    #[test]
    fn get_random_timeslot() {
        let first_result: SlotNames = rand::random();
        let mut next_result: SlotNames = rand::random();
        let mut ptr: i8 = 0;

        while first_result == next_result && ptr < 7 {
            ptr += 1;
            next_result = rand::random();
        }

        assert_ne!(first_result, next_result);
    }

    #[test]
    fn get_english_for_slotname() {
        let mut slotname_enum: SlotNames = SlotNames::EarlyMorning;
        assert_eq!(slotname_enum.to_string(), "early morning");

        slotname_enum = SlotNames::MidMorning;
        assert_eq!(slotname_enum.to_string(), "mid-morning");
    }

    #[test]
    fn check_can_access_random_slotname() {
        let test = Profile::get_default_timeslots();

        assert!(!test[0].name.to_string().is_empty());
        println!(
            "{} | {} | {}",
            test[0].name.to_string(),
            test[1].name.to_string(),
            test[2].name.to_string()
        );

        assert_eq!(test[0].name.to_string(), "late morning");
        assert_eq!(test[1].name.to_string(), "midday");
        assert_eq!(test[2].name.to_string(), "early afternoon");
    }
} // mod suite
// ---- end of file ----
