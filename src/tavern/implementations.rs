// ---- implementations  ----
pub mod List {
    use strum::{EnumString, IntoEnumIterator, VariantArray, VariantMetadata};
    use inflector::string::singularize::to_singular;
    use std::fmt;
    use is_vowel::*;

    use crate::tavern::enums::List::*;
    use crate::tavern::structs::List::*;
    use crate::tavern::traits::List::*;
    use crate::tavern::functions::*;

    impl AppFn for App {
        fn get_version(&self) -> String {
            format!(
                "{}.{}.{}",
                self.version_major, self.version_minor, self.version_fix
            )
        }
    }

    impl App {
        pub fn new() -> Self {
            App {
                name: "Fantasy Tavern Maker".into(),
                version_major: 0,
                version_minor: 6,
                version_fix: 0,
            }
        }
    }

    impl ToCapitalized for str {
        fn to_capitalized(&self) -> String {
            match self.len() {
                0 => String::new(),
                _ => {
                    let mut s = String::with_capacity(self.len());
                    s.extend(self.chars().next().unwrap().to_uppercase());
                    s.extend(self.chars().skip(1).flat_map(|c| c.to_lowercase()));
                    s
                }
            }
        }
    }

    impl PBHouse {
        pub fn new() -> Self {
            let eql = get_establishment_quality();
            let new_name = get_name();
            PBHouse {
                name: new_name.clone(),
                mood: get_mood(),
                lighting: get_lighting(),
                smells: get_smells(),
                size: get_pb_house_size(),
                establishment_quality: eql.clone(),
                posted_sign: get_posted_sign(),
                house_drink: get_house_drink(eql.level),
                house_dish: get_house_dish(eql.level),
                establishment_history_notes: get_establishment_history_notes(&new_name),
                redlight_services: get_redlight_services(),
            }
        }

        pub fn general_info(&self) -> Vec<String> {
            let mut pb_house_desc: Vec<String> = Vec::with_capacity(22);
            // ---
            let mut first_char = self
                .mood
                .to_string()
                .chars()
                .nth(0)
                .expect("This should be a single character");

            let prep = if first_char.is_romance_vowel() {
                "an"
            } else {
                "a"
            };

            let para1: String = format!(
                "'*The {}*' is the local Pub and Bed House for travellers in this area.
            The {}-quality establishment would be considered {}, with {} tables.",
                self.name,
                trim_whitespace(enum_string_to_phase(
                    self.establishment_quality.level.to_string()
                )),
                trim_whitespace(enum_string_to_phase(self.size.size_description.to_string())),
                self.size.table_count
            );
            pb_house_desc.push(para1);

            let bed_type_name = if self.size.common_bed_count == 1 {
                to_singular(&tidy(self.size.common_bed_type.to_string()))
            } else {
                tidy(self.size.common_bed_type.to_string())
            };
            let para2: String = format!(
                "It has {} {} in the common room and {} private rooms.
            Rooms are *{}* per day, and meals are *{}* per day.",
                self.size.common_bed_count,
                bed_type_name,
                self.size.private_room_count,
                self.establishment_quality.rooms,
                self.establishment_quality.meals
            );
            pb_house_desc.push(para2);

            let para3: String = format!(
            "As you enter, the air is full of the scents of {}. The current patrons seem to be {prep} {} bunch, {}. {}",
            self.smells, self.mood, self.lighting, self.posted_sign.clone()
        );
            pb_house_desc.push(para3);

            let para4: String = format!(
                "The menu has the usual standard fare posted.
            The House specialty beverage is {}, for {},
            while the House specialty dish is {}, for {}.",
                self.house_drink.desc,
                self.house_drink.price,
                self.house_dish.desc,
                self.house_dish.price
            );
            pb_house_desc.push(para4);

            // ---
            pb_house_desc
        }
    }

    impl fmt::Display for PBHouse {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}",
                "\n-----                        Player Blurb                        -----"
            )?;
            for line in &self.general_info() {
                write!(f, "{}", line)?;
            }

            write!(
                f,
                "{}",
                "\n -----                          DM Notes                          -----"
            )?;
            for line in &self.establishment_history_notes {
                write!(f, "{}", line)?;
            }

            for line in &self.redlight_services {
                write!(f, "{}", line)?;
            }

            // for line in &self.staff_and_customers() {
            //     write!(f, "{}", line)?;
            // }

            Ok(())
        }
    }
}

// ---- end of file ----
