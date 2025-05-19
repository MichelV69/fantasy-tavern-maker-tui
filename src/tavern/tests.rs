// ---- start of tests ----

mod suite {
    use tracing::{Level, event};
    use pretty_assertions::{assert_eq, assert_ne};
    use crate::tavern::enums::list::RSLCode;
    use crate::tavern::functions::get_red_light_services_list;
    use crate::tavern::structs::list::RedlightService;
    use crate::tavern::traits::list::DiceSize;

    #[test]
    fn can_get_a_service_dc() {
        event!(Level::INFO, "can_get_a_service_dc");

        let mut test_case: RedlightService = RedlightService { service: RSLCode::None, dc:0 };
        test_case.service = RSLCode::Brothel;
        test_case.dc = RSLCode::get_dc(&test_case.service);

        assert!( test_case.dc > 10)
    }

    #[test]
    fn redlight_service_honors_display() {
        event!(Level::INFO, "redlight_service_honors_display");

        let mut test_case: RedlightService = RedlightService { service: RSLCode::None, dc:0 };
        test_case.service = RSLCode::PitFighting;
        test_case.dc = 13;
        let expected_display_text = " * Pit fighting (access DC 13+)";

        assert_eq!(expected_display_text, test_case.display())
    }

    #[test]
    fn test_make_many_red_light_services(){
        event!(Level::INFO, "test_make_many_red_light_services");

        for _ in 0..10 {
            let rsl_list = get_red_light_services_list();
            match rsl_list {
                Some(list) => {
                    let redlight_services: Vec<RedlightService> = list;
                    let how_many_rsl = redlight_services.iter().count();
                    println!("DEBUG >>> how_many_rsl: {:#?}", how_many_rsl);
                    assert!(1 <= how_many_rsl);
                },
                None => {
                    println!("DEBUG >>> rsl_list is None");
                }
            }

        }
    }
    
    #[test]
    fn test_rsl_list_sorted_by_dc(){
        event!(Level::INFO, "test_rsl_list_sorted_by_dc");
        let mut first: RedlightService = RedlightService { service: RSLCode::None, dc:0 };
        first.service = RSLCode::Brothel;
        first.dc = 7;

        let mut second: RedlightService = RedlightService { service: RSLCode::None, dc:0 };
        second.service = RSLCode::Gambling;
        second.dc = 11;

        let mut third: RedlightService = RedlightService { service: RSLCode::None, dc:0 };
        third.service = RSLCode::Gambling;
        third.dc = 11;
        
        let mut rsl_list = vec![second, third, first ];
        rsl_list.sort_by(|a, b| a.dc.cmp(&b.dc));
        
        assert!(rsl_list[0].dc <= rsl_list[1].dc);
        assert!(rsl_list[1].dc <= rsl_list[2].dc);
    }
} // mod suite