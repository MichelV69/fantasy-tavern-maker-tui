// ---- all the uses all the time
use cursive::{Cursive, views::Dialog};
use dirs::download_dir;
use std::fs::File;
use std::io::Write;

// --- my stuff ---
use crate::{
    fn_make_pbhouse::make_pbhouse,
    npc::build::Profile,
    tavern::structs::list::{App, PBHouse},
};

pub fn save_pbhouse_to_file(s: &mut Cursive, pbh: PBHouse, app: App, npc_list: Vec<Profile>) {
    s.pop_layer();

    let path_name = match download_dir() {
        Some(value) => value,
        None => panic!("Problem determining user download dir!"),
    };

    let fname = format!(
        "{}/{}-{}.md",
        path_name.display(),
        app.name,
        pbh.name.to_lowercase().replace(" ", "_")
    );
    let file_open_result = File::create(fname);

    let mut file_handle = match file_open_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    let file_write_result = write!(file_handle, "{}", format_args!("\n \n {} \n \n", pbh));
    match file_write_result {
        Ok(file) => file,
        Err(error) => panic!("Problem writing to the file: {error:?}"),
    };

    s.add_layer(
        Dialog::text(format!("{} - saved {} to disk", &app.name, pbh.name))
            .title(&app.name)
            .button("Roll another", move |s| make_pbhouse(s, app.clone()))
            .button("Finish", |s| s.quit()),
    );
}
// --- eof ---
