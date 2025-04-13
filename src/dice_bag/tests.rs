// ---- start of tests ----

mod suite {
    use crate::dice_bag::tower::{DiceResult, RollDice};
    use tracing::{Level, event};

    #[test]
    fn does_flip_coin() {
        let request: &str = "1d2";
        let resulting_roll = <DiceResult as RollDice>::from_string(request);
        let roll_value: i16 = resulting_roll.get_total();

        event!(Level::INFO, "roll_value[{}]", roll_value);
        println!("roll_value[{}]", roll_value);
        debug_assert!((1..=2).contains(&roll_value));
    }

    #[test]
    fn rolls_1d4() {
        let request: &str = "1d4";
        let resulting_roll = <DiceResult as RollDice>::from_string(request);
        let roll_value: i16 = resulting_roll.get_total();

        event!(Level::INFO, "from_string[{}]", roll_value);
        println!("roll_value[{}]", roll_value);
        debug_assert!((1..=4).contains(&roll_value));
    }

    #[test]
    fn rolls_22d8() {
        let request: &str = "22d8";
        let resulting_roll = <DiceResult as RollDice>::from_string(request);
        let roll_value: i16 = resulting_roll.get_total();

        event!(Level::INFO, "from_string[{}]", roll_value);
        println!("roll_value[{}]", roll_value);
        for this_roll in resulting_roll.get_rolls() {
            println!("this_roll[{}]", &this_roll);
            debug_assert!((1..=8).contains(&this_roll));
        }
    }

    #[test]
    fn rolls_respect_pos_modifiers() {
        let request: &str = "1d6+3";

        for _ in 1..99 {
            let resulting_roll = <DiceResult as RollDice>::from_string(request);
            let roll_value: i16 = resulting_roll.get_total();
            let mod_total: i16 = resulting_roll.get_mod_total();

            event!(Level::INFO, "from_string[{}]", roll_value);
            println!("roll_value[{}]", roll_value);
            println!("mod_total[{}]", mod_total);
            debug_assert!((4..=9).contains(&roll_value)); //21
        }
    }

    #[test]
    fn rolls_respect_neg_modifiers() {
        let request: &str = "1d6-3";

        for _ in 1..99 {
            let resulting_roll = <DiceResult as RollDice>::from_string(request);
            let roll_value: i16 = resulting_roll.get_total();
            let mod_total: i16 = resulting_roll.get_mod_total();

            event!(Level::INFO, "from_string[{}]", roll_value);
            println!("roll_value[{}]", roll_value);
            println!("mod_total[{}]", mod_total);
            debug_assert!((-2..=3).contains(&roll_value)); //21
        }
    }

    #[test]
    fn rolls_inline_3d4plus2() {
        let request: &str = "rolls_inline_3d4plus2: [3d4+2]. <<== ";
        let resulting_text = <DiceResult as RollDice>::inline_replace(request);

        println!("resulting_text [{}]", resulting_text);
        debug_assert!(request != resulting_text);
    }

    #[test]
    fn rolls_inline_4d6minus3() {
        let request: &str = "rolls_inline_4d6minus3: [4d6-3]. <<== ";
        let resulting_text = <DiceResult as RollDice>::inline_replace(request);

        println!("resulting_text [{}]", resulting_text);
        debug_assert!(request != resulting_text);
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
