mod suite {

    use std::mem;

    use tracing::{Level, event};

    use crate::narrative_time_manager::ntm::{self, SlotNames};

    #[test]
    fn number_of_timeslots() {
        let target_value : usize = 17;
        assert_eq!(target_value, mem::variant_count::<SlotNames>());
    }

    #[test]
    fn check_noon() {
        let test_data = ntm::load();
        let test_pointer = 5;
        assert_eq!(test_data[test_pointer].name, SlotNames::Midday);
    }

    #[test]
    fn check_sunset() {
        let test_data = ntm::load();
        let test_pointer = 10;
        assert_eq!(test_data[test_pointer].name, SlotNames::Sunset);
    }
} // mod suite
// ---- end of file ----
