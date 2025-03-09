// ---- start of file ----
pub mod Tower {
    use rand::distributions::{Distribution, Uniform};
    use rand::prelude::*;
    use std::str::Split;
    use tracing::{event, Level};

    pub struct DiceResult {
        request: String,
        rolls: Vec<i8>,
        total_mod: i8,
        total_roll: i8,
    }

    pub trait RollDice {
        fn get_request(&self) -> String;
        fn get_total(&self) -> i8;
        fn get_mod_total(&self) -> i8;
        fn get_rolls(&self) -> Vec<i8>;
        fn from_string(request: &str) -> DiceResult;
        fn inline_replace(human_readable: &str) -> String;
        fn from_pool(request: &str) -> DiceResult;
    }

    impl RollDice for DiceResult {
        fn get_request(&self) -> String {
            self.request.clone()
        }

        fn get_total(&self) -> i8 {
            self.total_roll
        }

        fn get_mod_total(&self) -> i8 {
            self.total_mod
        }

        fn get_rolls(&self) -> Vec<i8> {
            self.rolls.clone()
        }

        fn from_string(request: &str) -> DiceResult {
            // eg "5d12-6"

            if request.contains("coin") && request.contains("flip") {
                let new_roll_request = {
                    RollRequest {
                        die_requested: DiceBag::Coin,
                        number_rolls: 1,
                        modifer_list: vec![0],
                    }
                };

                return process_roll_request(new_roll_request);
            }

            // ---
            let buffer1: Vec<String> = request.split('d').map(|s| s.to_string()).collect();
            let die_count = buffer1[0].parse::<i8>().unwrap();
            let die_size = buffer1[1]
                .split(['+', '-'])
                .map(|i| i.parse::<i8>().unwrap())
                .collect::<Vec<_>>()[0];

            // ----
            let mut mod_list: Vec<i8> = vec![];
            if buffer1[1].contains(['+', '-']) {
                let mut buffer: String = "".to_string();
                for c in buffer1[1].chars() {
                    if ['+', '-'].contains(&c) && !buffer.is_empty() {
                        mod_list.push(buffer.parse::<i8>().unwrap());
                        buffer = "".to_string();
                    }
                    buffer += &c.to_string();
                }
                if !buffer.is_empty() {
                    mod_list.push(buffer.parse::<i8>().unwrap());
                }
            }

            // ---
            if mod_list.len() > 1 {
                mod_list = mod_list[1..].to_vec();
            }

            // ---
            let mut mod_total: i8 = 0;
            for v in &mod_list {
                mod_total += v;
            }

            event!(
                Level::INFO,
                "die_size[{:#?}] die_count[{:#?}] mod_list[{:#?}]",
                die_size,
                die_count,
                mod_list
            );

            let mut panic_reason = format!(
                "Unsupported die-size description c{} d{}  m'{:#?}'.",
                die_count, die_size, mod_list
            );
            let requested_size = match die_size {
                2 => DiceBag::Coin,
                4 => DiceBag::D4,
                6 => DiceBag::D6,
                8 => DiceBag::D8,
                10 => DiceBag::D10,
                12 => DiceBag::D12,
                20 => DiceBag::D20,
                _ => panic!("{}", &panic_reason),
            };

            let new_roll_request = {
                RollRequest {
                    die_requested: requested_size,
                    number_rolls: die_count,
                    modifer_list: mod_list,
                }
            };

            process_roll_request(new_roll_request)
        }

        fn inline_replace(human_readable: &str) -> String {
            // human_readable expect to find "there are [5d12+6] bad guys"
            // break out the dice string and encapsulate into new_roll_request
            let buffer1: Vec<String> = human_readable.split('[').map(|s| s.to_string()).collect();
            // "there are [","5d12+6] bad guys"
            let buffer2: Vec<String> = buffer1[1].split(']').map(|s| s.to_string()).collect();
            // "5d12+6"," bad guys"
            let desired_roll = &buffer2[0];
            // make the die roll
            let wrapped_roll = Self::from_string(desired_roll);
            // unwrap die roll
            let roll_value = wrapped_roll.get_total();
            // replace the dice string with the roll result roll_value
            let needle = format!("[{}]", &desired_roll);
            // return the modified string
            human_readable.replace(&needle, &roll_value.to_string())
        }

        fn from_pool(request: &str) -> DiceResult {
            let buffer1: Vec<String> = request.split('|').map(|s| s.to_string()).collect();

            let dice_string: String = buffer1[0].clone();
            let success_check: i8 = buffer1[1].parse::<i8>().expect("should be an i8");

            let buffer2: Vec<String> = dice_string.split('d').map(|s| s.to_string()).collect();

            let die_count: i8 = buffer2[0].parse::<i8>().expect("should be N of NdX");
            let die_size: i8 = buffer2[1].parse::<i8>().expect("should be X of NdX");

            let wrapped_roll = Self::from_string(&format!("{}d{}", die_count, die_size));
            let list_rolls = &wrapped_roll.get_rolls();

            let mut success_count: i8 = 0;
            for roll in list_rolls {
                if *roll >= success_check {
                    success_count += 1;
                }
            }
            // --- --- ---
            DiceResult {
                request: format!(
                    "requested {}d{}, Success {}+, giving {} successes",
                    die_count, die_size, success_check, success_count
                ),
                rolls: list_rolls.to_vec(),
                total_mod: 0,
                total_roll: success_count,
            }
        }
    } // impl DiceResult

    struct RollRequest {
        die_requested: DiceBag,
        number_rolls: i8,
        modifer_list: Vec<i8>,
    }

    enum DiceBag {
        Coin,
        D2,
        D4,
        D6,
        D8,
        D10,
        D12,
        D20,
    }

    fn process_roll_request(request: RollRequest) -> DiceResult {
        let mut rng = rand::thread_rng();
        let mut roll_list: Vec<i8> = [].to_vec();

        let die2 = Uniform::new(0, 2);
        let die4 = Uniform::new(0, 4);
        let die6 = Uniform::new(0, 6);
        let die8 = Uniform::new(0, 8);
        let die10 = Uniform::new(0, 10);
        let die12 = Uniform::new(0, 12);
        let die20 = Uniform::new(0, 20);

        for index in 0..request.number_rolls {
            let low_roll_val: i8 = match request.die_requested {
                DiceBag::Coin | DiceBag::D2 => rng.sample::<i8, _>(die2),
                DiceBag::D4 => rng.sample::<i8, _>(die4),
                DiceBag::D6 => rng.sample::<i8, _>(die6),
                DiceBag::D8 => rng.sample::<i8, _>(die8),
                DiceBag::D10 => rng.sample::<i8, _>(die10),
                DiceBag::D12 => rng.sample::<i8, _>(die12),
                DiceBag::D20 => rng.sample::<i8, _>(die20),
            };
            let roll_val = low_roll_val + 1;
            event!(Level::INFO, "roll_val[{}]", roll_val);

            roll_list.push(roll_val);
        }

        // modifer_list: Vec<i8>
        let mut mod_total = 0;
        for v in request.modifer_list {
            mod_total += v;
        }

        let mut roll_total = 0;
        for roll in &roll_list {
            roll_total += roll;
        }
        roll_total += mod_total;

        DiceResult {
            request: "no_roll".to_string(),
            rolls: roll_list,
            total_mod: mod_total,
            total_roll: roll_total,
        }
    }
} // pub mod dicebag{

// ---- start of tests ----
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tower::*;
    use tracing::{event, Level};

    #[test]
    fn does_flip_coin() {
        let request: &str = "1d2";
        let resulting_roll = <Tower::DiceResult as RollDice>::from_string(request);
        let roll_value: i8 = resulting_roll.get_total();

        event!(Level::INFO, "roll_value[{}]", roll_value);
        println!("roll_value[{}]", roll_value);
        debug_assert!((1..=2).contains(&roll_value));
    }

    #[test]
    fn rolls_1d4() {
        let request: &str = "1d4";
        let resulting_roll = <Tower::DiceResult as RollDice>::from_string(request);
        let roll_value: i8 = resulting_roll.get_total();

        event!(Level::INFO, "from_string[{}]", roll_value);
        println!("roll_value[{}]", roll_value);
        debug_assert!((1..=4).contains(&roll_value));
    }

    #[test]
    fn rolls_22d8() {
        let request: &str = "22d8";
        let resulting_roll = <Tower::DiceResult as RollDice>::from_string(request);
        let roll_value: i8 = resulting_roll.get_total();

        event!(Level::INFO, "from_string[{}]", roll_value);
        println!("roll_value[{}]", roll_value);
        for this_roll in resulting_roll.get_rolls() {
            debug_assert!((1..=8).contains(&this_roll));
        }
    }

    #[test]
    fn rolls_respect_pos_modifiers() {
        let request: &str = "1d6+3";

        for i in 1..99 {
            let resulting_roll = <Tower::DiceResult as RollDice>::from_string(request);
            let roll_value: i8 = resulting_roll.get_total();
            let mod_total: i8 = resulting_roll.get_mod_total();

            event!(Level::INFO, "from_string[{}]", roll_value);
            println!("roll_value[{}]", roll_value);
            println!("mod_total[{}]", mod_total);
            debug_assert!((4..=9).contains(&roll_value)); //21
        }
    }

    #[test]
    fn rolls_respect_neg_modifiers() {
        let request: &str = "1d6-3";

        for i in 1..99 {
            let resulting_roll = <Tower::DiceResult as RollDice>::from_string(request);
            let roll_value: i8 = resulting_roll.get_total();
            let mod_total: i8 = resulting_roll.get_mod_total();

            event!(Level::INFO, "from_string[{}]", roll_value);
            println!("roll_value[{}]", roll_value);
            println!("mod_total[{}]", mod_total);
            debug_assert!((-2..=3).contains(&roll_value)); //21
        }
    }

    #[test]
    fn rolls_inline_3d4plus2() {
        let request: &str = "rolls_inline_3d4plus2: [3d4+2]. <<== ";
        let resulting_text = <Tower::DiceResult as RollDice>::inline_replace(request);

        println!("resulting_text [{}]", resulting_text);
        debug_assert!(request != resulting_text);
    }

    #[test]
    fn rolls_inline_4d6minus3() {
        let request: &str = "rolls_inline_4d6minus3: [4d6-3]. <<== ";
        let resulting_text = <Tower::DiceResult as RollDice>::inline_replace(request);

        println!("resulting_text [{}]", resulting_text);
        debug_assert!(request != resulting_text);
    }
}
// ---- end of file ----
