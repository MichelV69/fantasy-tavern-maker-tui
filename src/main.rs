#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

use cursive::Cursive;
use cursive::view::Resizable;
use cursive::view::Scrollable;
use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::TextView;
use dirs::download_dir;
use npc::Build::NpcTypeCode;
use npc::Build::Profile as npc_Profile;
use tavern::structs::List::App;
use tavern::structs::List::PBHouse;
use tavern::traits::List::AppFn;
use tracing::info;

use tavern::functions::l1_heading;

use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;

// --- my stuff ---
mod dice_bag;
mod npc;
mod tavern;

// todo!("add types of mead to the drink list");

// --- local cli code
fn main() {
    let mut app: App = App::new();
    app.name = "fantasy-tavern-maker-tui".into();
    app.version_build = 96;
    app.version_major = 0;
    app.version_minor = 8;
    app.version_fix = 01;

    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text(format!("Welcome to {} ({})", &app.name, &app.get_version()))
            .title(&app.name)
            .button("Create a new P&B House?", move |s| {
                get_new_pbhouse(s, app.clone())
            })
            .button("Quit", |s| s.quit()),
    );

    siv.run()
}

fn get_new_pbhouse(s: &mut Cursive, app: App) {
    let pbh = PBHouse::new();
    let mut gm_text: String = "".into();
    let mut player_text: String = "".into();

    //---
    let dialog_title = format!("P&B House: the {}", &pbh.name);
    for line in &pbh.general_info() {
        player_text += line;
    }

    gm_text += &l1_heading("Establishment History Notes".to_string());
    for line in &pbh.establishment_history_notes {
        gm_text += line;
    }

    gm_text += &l1_heading("Redlight Services".to_string());

    if !pbh.redlight_services.is_empty() {
        for line in &pbh.redlight_services {
            gm_text += line;
        }
    } else {
        gm_text += "_<none>_";
    }

    //--- NPCs present
    // ("Staff", "Owner")
    let mut npc_list: Vec<npc_Profile> = vec![];
    let mut new_npc: npc_Profile = npc_Profile::new();
    new_npc.npc_type = NpcTypeCode::Staff;
    new_npc.task_description = "Owner".into();
    new_npc.random_appearance(&app);
    new_npc.set_random_quirk_emotional(&app);
    new_npc.set_random_schticks_attributes(&app);

    // modest || large || massive => ("Staff", "Cook")
    // large || massive => ("Staff", "Head Server")
    // massive => ("Staff", "Bouncer")
    // notablePatronsList ... #dice based on Establishment.size
    // redlight_services ?? "specific" NPCs such as extra bouncer, wealthy gladiator, cardshark, healer ??

    //---
    s.pop_layer();
    let app1 = app.clone();
    let app2 = app.clone();

    s.add_layer(
        Dialog::new()
            .title(dialog_title)
            .content(
                LinearLayout::horizontal()
                    .child(Dialog::text(gm_text).title("GM Notes").fixed_width(42))
                    .child(
                        Dialog::text(player_text)
                            .title("Player Notes")
                            .fixed_width(42)
                            .scrollable()
                            .scroll_y(true),
                    ),
            )
            .button("Save to file", move |s| {
                save_pbhouse_to_file(s, pbh.clone(), app1.clone())
            })
            .button("Roll another", move |s| get_new_pbhouse(s, app2.clone()))
            .h_align(cursive::align::HAlign::Center)
            .button("Quit", |s| s.quit()),
    );
}

fn save_pbhouse_to_file(s: &mut Cursive, pbh: PBHouse, app: App) {
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
            .button("Roll another", move |s| get_new_pbhouse(s, app.clone()))
            .button("Finish", |s| s.quit()),
    );
}

// --- eof ---
