/// external functions used by the mod
/// accessible to all class/struct/impl
///

pub mod fnset {
    use std::{
        fs::{self, read_to_string, File},
        io::Read,
        path::{Path, PathBuf},
    };

    use dirs::document_dir;
    use tracing::{event, Level};
    use crate::structs::List::App;

    pub struct RollTable {
        pub low: i8,
        pub high: i8,
        pub result: String,
    }

    pub fn read_psv_file(psv_file: &str, app: &App) -> Vec<(i8, String)> {
        let mut table_set: Vec<(i8, String)> = Vec::with_capacity(42);

        let get_dir_result = match document_dir() {
            Some(path) => path,
            None => panic!("Should have the tables folder installed!")
        };

        let psv_file_path: PathBuf = format!("{}/{}/{}", get_dir_result.display(), app.name , psv_file).into();

        let read_result = fs::read_to_string(&psv_file_path);
        let mut file_content = match read_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: [{error:?} : {psv_file_path:?}]"),
        };

        let table_rows = file_content.lines().map(|line| line.trim());

        for row in table_rows {
            let probability: i8 = row.split("|").collect::<Vec<_>>()[0]
                .trim()
                .parse()
                .expect("please give me correct string number!");
            let description: String = row.split("|").collect::<Vec<_>>()[1].trim().to_string();
            table_set.push((probability, description));
        }

        return table_set;
    } // read_psv_file

} // mod fnset

// ---- end of file ----
