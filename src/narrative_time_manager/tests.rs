mod suite {
    use std::mem;

    use tracing::{Level, event};

    use crate::narrative_time_manager::ntm::{self, SlotNames};

    #[test]
    fn number_of_timeslots() {
        debug_assert_eq!(11, mem::variant_count::<SlotNames>);
    }

    fn check_noon() {
        let test_data = ntm::load();
        debug_assert_eq!(test_data[1].name, SlotNames::Midday);
    }
} // mod suite
// ---- end of file ----
