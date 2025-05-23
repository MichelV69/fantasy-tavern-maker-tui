// ---- traits  ----
pub mod list {

    // from https://stackoverflow.com/questions/38342805/how-do-i-collect-from-multiple-iterator-types#
    pub trait ToCapitalized {
        fn to_capitalized(&self) -> String;
    }

    pub trait AppFn {
        fn get_version(&self) -> String;
    }

    pub trait DiceSize {
        fn get_dc(&self) -> i16;
    }
}

// ---- end of file ----
