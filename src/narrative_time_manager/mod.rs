// ----- start of file -----
/// Divides the day into "event slots" to allow narrative
/// description and plot sequences.
pub mod ntm {

    pub enum hour  [00...23];
    pub enum slot_names [Twilight, Dawn, 
        EarlyMorning, MidMorning, LateMorning,
        Midday,
        EarlyAfternoon, MidAfternoon, LateAfternoon,
        Dusk, Sunset,
        EarlyEvening, MidEvening,
        Night, Midnight,
        LateNight, LongDark
        ]

    #[derive]
    pub struct TimeSlot {
        pub start: hour;
        pub duration: i8;
        pub name: slot_names;
    }

    pub fn load -> Vec<TimeSlot> {
        let mut data Vec<TimeSlot> = {};
        let mut record TimeSlot = {};
        
        record.start = hour::5;
        record.duration = 1;
        record.name = slot_names::Twilight;
        data.append(record);

        record.start = hour::6;
        record.duration = 1;
        record.name = slot_names::Dawn;
        data.append(record);

        record.start = hour::7
        record.duration = 2;
        record.name = slot_names::EarlyMorning;
        data.append(record);

        record.start = hour::9
        record.duration = 2;
        record.name = slot_names::MidMorning;
        data.append(record);

        record.start = hour::11
        record.duration = 1;
        record.name = slot_names::LateMorning;
        data.append(record);



    }

} //pub mod ntm
mod lib;
#[cfg(test)]
mod tests;