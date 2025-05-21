// ---- start of tests ----

mod suite {
    use crate::dice_bag::tower::{DiceResult, RollDice};
    use pretty_assertions::{assert_eq, assert_ne};
    use tracing::{Level, event};

    #[test]
    fn does_flip_coin() {
        let request: &str = "1d2";
        let valid_scope = (1..=2);

        for _ in (0..valid_scope.len() as i16 * 2) {
            let resulting_roll = <DiceResult as RollDice>::from_string(request);
            let roll_value = resulting_roll.get_total();
            event!(Level::INFO, "roll_value[{}]", roll_value);
            assert!(valid_scope.contains(&roll_value));
        }
    }

    #[test]
    fn rolls_1d4() {
        let request: &str = "1d4";
        let valid_scope = (1..=4);

        for _ in (0..valid_scope.len() as i16 * 2) {
            let resulting_roll = <DiceResult as RollDice>::from_string(request);
            let roll_value = resulting_roll.get_total();
            event!(Level::INFO, "roll_value[{}]", roll_value);
            assert!(valid_scope.contains(&roll_value));
        }
    }

    #[test]
    fn rolls_4d8() {
        let request: &str = "4d8";
        let valid_scope = (4..=32);

        for _ in (0..valid_scope.len() as i16 * 2) {
            let resulting_roll = <DiceResult as RollDice>::from_string(request);
            let roll_value = resulting_roll.get_total();
            event!(Level::INFO, "roll_value[{}]", roll_value);
            assert!(valid_scope.contains(&roll_value));
        }
    }

    #[test]
    fn rolls_respect_pos_modifiers() {
        let request: &str = "1d6+3";
        let valid_scope = (4..=9);

        for _ in (0..valid_scope.len() as i16 * 2) {
            let resulting_roll = <DiceResult as RollDice>::from_string(request);
            let roll_value = resulting_roll.get_total();
            event!(Level::INFO, "roll_value[{}]", roll_value);
            assert!(valid_scope.contains(&roll_value));
        }
    }

    #[test]
    fn rolls_respect_neg_modifiers() {
        let request: &str = "1d6-3";
        let valid_scope = (-2..=3);

        for _ in (0..valid_scope.len() as i16 * 2) {
            let resulting_roll = <DiceResult as RollDice>::from_string(request);
            let roll_value = resulting_roll.get_total();
            event!(Level::INFO, "roll_value[{}]", roll_value);
            assert!(valid_scope.contains(&roll_value));
        }
    }

    #[test]
    fn rolls_inline_3d4plus2() {
        let request: &str = "rolls_inline_3d4plus2: [3d4+2]. <<== ";
        let resulting_text = <DiceResult as RollDice>::inline_replace(request);

        println!("resulting_text [{}]", resulting_text);
        assert_ne!(request, resulting_text);
    }

    #[test]
    fn rolls_inline_4d6minus3() {
        let request: &str = "rolls_inline_4d6minus3: [4d6-3]. <<== ";
        let resulting_text = <DiceResult as RollDice>::inline_replace(request);

        println!("resulting_text [{}]", resulting_text);
        assert_ne!(request, resulting_text);
    }

    #[test]
    fn rolls_inline_nothing_to_do() {
        let request: &str = "rolls_inline_nothing_to_do. Still nothing. ";
        let resulting_text = <DiceResult as RollDice>::inline_replace(request);

        println!("resulting_text [{}]", resulting_text);
        assert_eq!(request, resulting_text);
    }
} // mod suite
// ---- end of file ----
