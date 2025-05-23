// ----- start of file -----
/// Divides the day into "event slots" to allow narrative
/// description and plot sequences.
pub mod ntm {

    use crate::text_postproc::tpp::enum_string_to_phrase;
    use rand::Rng;
    use rand::distr::{Distribution, StandardUniform};

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Hour {
        H00,
        H01,
        H02,
        H03,
        H04,
        H05,
        H06,
        H07,
        H08,
        H09,
        H10,
        H11,
        H12,
        H13,
        H14,
        H15,
        H16,
        H17,
        H18,
        H19,
        H20,
        H21,
        H22,
        H23,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SlotNames {
        Twilight,
        Sunrise,        // 1 *
        EarlyMorning,   // 2 *
        MidMorning,     // 3 *
        LateMorning,    // 4 * *
        Midday,         // 5 * *
        EarlyAfternoon, // 6   *
        MidAfternoon,   // 7   *
        LateAfternoon,  // 8   * *
        Dusk,           // 9   * *
        Sunset,         // 10    *
        LateEvening,    // 11    *
        EarlyEvening,   // 12 *  *
        MidEvening,     // 13 *  *
        Night,          // 14 *
        Midnight,       // 15 *
        LateNight,      // 16 *
        LongDark,
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct TimeSlot {
        pub start: Hour,
        pub duration: i8,
        pub name: SlotNames,
    }

    pub fn load() -> Vec<TimeSlot> {
        let mut data: Vec<TimeSlot> = Vec::with_capacity(24);
        let mut record: TimeSlot = TimeSlot {
            start: Hour::H00,
            duration: 0,
            name: SlotNames::Midnight,
        };

        record.start = Hour::H00;
        record.duration = 1;
        record.name = SlotNames::Midnight;
        data.push(record);

        record.start = Hour::H01;
        record.duration = 2;
        record.name = SlotNames::LateNight;
        data.push(record);

        record.start = Hour::H03;
        record.duration = 2;
        record.name = SlotNames::LongDark;
        data.push(record);

        record.start = Hour::H05;
        record.duration = 1;
        record.name = SlotNames::Twilight;
        data.push(record);

        record.start = Hour::H06;
        record.duration = 1;
        record.name = SlotNames::Sunrise;
        data.push(record);

        record.start = Hour::H07;
        record.duration = 2;
        record.name = SlotNames::EarlyMorning;
        data.push(record);

        record.start = Hour::H09;
        record.duration = 1;
        record.name = SlotNames::MidMorning;
        data.push(record);

        record.start = Hour::H10;
        record.duration = 2;
        record.name = SlotNames::LateMorning;
        data.push(record);

        record.start = Hour::H12;
        record.duration = 1;
        record.name = SlotNames::Midday;
        data.push(record);

        record.start = Hour::H13;
        record.duration = 2;
        record.name = SlotNames::EarlyAfternoon;
        data.push(record);

        record.start = Hour::H15;
        record.duration = 1;
        record.name = SlotNames::MidAfternoon;
        data.push(record);

        record.start = Hour::H16;
        record.duration = 2;
        record.name = SlotNames::LateAfternoon;
        data.push(record);

        record.start = Hour::H18;
        record.duration = 1;
        record.name = SlotNames::Dusk;
        data.push(record);

        record.start = Hour::H19;
        record.duration = 1;
        record.name = SlotNames::Sunset;
        data.push(record);

        record.start = Hour::H20;
        record.duration = 1;
        record.name = SlotNames::EarlyEvening;
        data.push(record);

        record.start = Hour::H21;
        record.duration = 1;
        record.name = SlotNames::MidEvening;
        data.push(record);

        record.start = Hour::H22;
        record.duration = 1;
        record.name = SlotNames::LateEvening;
        data.push(record);

        record.start = Hour::H23;
        record.duration = 1;
        record.name = SlotNames::Night;
        data.push(record);

        // send the result up the line
        data
    }

    impl Distribution<SlotNames> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SlotNames {
            let index: u8 = rng.random_range(0..=17);
            match index {
                0 => SlotNames::Midnight,
                1 => SlotNames::LateNight,
                2 => SlotNames::LongDark,
                3 => SlotNames::Twilight,
                4 => SlotNames::Sunrise,
                5 => SlotNames::EarlyMorning,
                6 => SlotNames::MidMorning,
                7 => SlotNames::LateMorning,
                8 => SlotNames::Midday,
                9 => SlotNames::EarlyAfternoon,
                10 => SlotNames::MidAfternoon,
                11 => SlotNames::LateAfternoon,
                12 => SlotNames::Dusk,
                13 => SlotNames::Sunset,
                14 => SlotNames::EarlyEvening,
                15 => SlotNames::MidEvening,
                16 => SlotNames::LateEvening,
                17 => SlotNames::Night,
                _ => unreachable!(),
            }
        }
    }

    impl ToString for SlotNames {
        fn to_string(&self) -> String {
            let sn_string = match self {
                SlotNames::Midnight => "Midnight",
                SlotNames::LateNight => "LateNight",
                SlotNames::LongDark => "LateNight",
                SlotNames::Twilight => "Twilight",
                SlotNames::Sunrise => "Sunrise",
                SlotNames::EarlyMorning => "EarlyMorning",
                SlotNames::MidMorning => "Mid-Morning",
                SlotNames::LateMorning => "LateMorning",
                SlotNames::Midday => "Midday",
                SlotNames::EarlyAfternoon => "EarlyAfternoon",
                SlotNames::MidAfternoon => "MidAfternoon",
                SlotNames::LateAfternoon => "LateAfternoon",
                SlotNames::Dusk => "Dusk",
                SlotNames::Sunset => "Sunset",
                SlotNames::EarlyEvening => "EarlyEvening",
                SlotNames::MidEvening => "MidEvening",
                SlotNames::LateEvening => "LateEvening",
                SlotNames::Night => "Night",
            };

            format!("{}", enum_string_to_phrase(sn_string.into()))
        }
    }
} //pub mod ntm

#[cfg(test)]
mod tests;
// ----- end of file -----
