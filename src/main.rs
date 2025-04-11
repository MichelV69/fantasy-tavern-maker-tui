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
use dice_bag::Tower::DiceResult;
use dice_bag::Tower::RollDice;
use dirs::download_dir;
use npc::Build::NpcTypeCode;
use npc::Build::Profile as npc_Profile;
use tavern::enums::List::EstablishmentQualityLevel;
use tavern::enums::List::SizeList;
use tavern::structs::List::App;
use tavern::structs::List::EstablishmentQuality;
use tavern::structs::List::PBHouse;
use tavern::structs::List::PBHouseSize;
use tavern::traits::List::AppFn;
use tracing::info;

use tavern::functions::l1_heading;

use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;
use std::path::absolute;
use std::process::Child;

// --- my stuff ---
mod dice_bag;
mod npc;
mod tavern;

// --- local cli code
fn main() {
    let mut app: App = App::new();
    app.name = "fantasy-tavern-maker-tui".into();
    app.version_build = 126;
    app.version_major = 0;
    app.version_minor = 9;
    app.version_fix = 1;

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
    let mut npc_staff_list: Vec<npc_Profile> = vec![];
    let mut npc_owner: npc_Profile = npc_Profile::new();
    npc_owner.npc_type = NpcTypeCode::Staff;
    npc_owner.task_description = "Owner".into();
    npc_owner.random_appearance(&app);
    npc_owner.set_random_quirk_emotional(&app);
    npc_owner.set_random_schticks_attributes(&app);
    npc_staff_list.push(npc_owner);

    if [SizeList::Modest, SizeList::Large, SizeList::Massive].contains(&pbh.size.size_description) {
        let mut npc_cook: npc_Profile = npc_Profile::new();
        npc_cook.npc_type = NpcTypeCode::Staff;
        npc_cook.task_description = "Cook".into();
        npc_cook.random_appearance(&app);
        npc_cook.set_random_quirk_emotional(&app);
        npc_cook.set_random_schticks_attributes(&app);
        npc_staff_list.push(npc_cook);

        if [SizeList::Large, SizeList::Massive].contains(&pbh.size.size_description) {
            let mut npc_server: npc_Profile = npc_Profile::new();
            npc_server.task_description = "Server".into();
            npc_server.npc_type = NpcTypeCode::Staff;
            npc_server.random_appearance(&app);
            npc_server.set_random_quirk_emotional(&app);
            npc_server.set_random_schticks_attributes(&app);
            npc_staff_list.push(npc_server);

            if [SizeList::Massive].contains(&pbh.size.size_description) {
                let request: &str = "1d6";

                if <DiceResult as RollDice>::from_string(request).get_total() < 4 {
                    let mut npc_cook: npc_Profile = npc_Profile::new();
                    npc_cook.npc_type = NpcTypeCode::Staff;
                    npc_cook.task_description = "Cook Helper".into();
                    npc_cook.random_appearance(&app);
                    npc_cook.set_random_quirk_emotional(&app);
                    npc_cook.set_random_schticks_attributes(&app);
                    npc_staff_list.push(npc_cook);
                }

                if <DiceResult as RollDice>::from_string(request).get_total() < 4 {
                    let mut npc_server: npc_Profile = npc_Profile::new();
                    npc_server.npc_type = NpcTypeCode::Staff;
                    npc_server.task_description = "Server Helper".into();
                    npc_server.random_appearance(&app);
                    npc_server.set_random_quirk_emotional(&app);
                    npc_server.set_random_schticks_attributes(&app);
                    npc_staff_list.push(npc_server);
                }

                let mut npc_bouncer: npc_Profile = npc_Profile::new();
                npc_bouncer.task_description = "Bouncer".into();
                npc_bouncer.npc_type = NpcTypeCode::Staff;
                npc_bouncer.random_appearance(&app);
                npc_bouncer.set_random_quirk_emotional(&app);
                npc_bouncer.set_random_schticks_attributes(&app);
                npc_staff_list.push(npc_bouncer);
            }
        }
    }

    // notablePatronsList ... #dice based on Establishment.size
    let mut npc_notable_patrons_list: Vec<npc_Profile> = vec![];
    let die_size: String = "1d4".into();
    //println!("\n --- \n die_size [{die_size}]");

    let mut roll_mod: i8 = match pbh.size.size_description {
        SizeList::Tiny => -1,
        SizeList::Small => 1,
        SizeList::Modest => 2,
        SizeList::Large => 3,
        SizeList::Massive => 4,
    };

    //println!("roll_mod [{roll_mod}]");

    roll_mod += match pbh.establishment_quality.level {
        EstablishmentQualityLevel::Squalid => -2,
        EstablishmentQualityLevel::Poor => -1,
        EstablishmentQualityLevel::Modest => 0,
        EstablishmentQualityLevel::Comfortable => 1,
        EstablishmentQualityLevel::Wealthy => 2,
        EstablishmentQualityLevel::Aristocratic => 3,
    };

    // println!("roll_mod [{roll_mod}]");
    let mut math_func: &str = "+";
    if roll_mod != roll_mod.abs() {
        math_func = "";
    }

    let mut roll_string: String = format!("{}{}{}", die_size, math_func.to_string(), roll_mod);
    //println!("roll_string [{roll_string}]");

    let npc_notable_patrons_count: i16 =
        <DiceResult as RollDice>::from_string(&roll_string).get_total();
    //println!("npc_notable_patrons_count [{npc_notable_patrons_count}]");

    for c in 1..npc_notable_patrons_count {
        let mut npc_patron: npc_Profile = npc_Profile::new();
        npc_patron.npc_type = NpcTypeCode::Patron;
        npc_patron.set_random_task_description(&app);
        npc_patron.random_appearance(&app);
        npc_patron.set_random_quirk_emotional(&app);
        npc_patron.set_random_schticks_attributes(&app);
        npc_notable_patrons_list.push(npc_patron);
    }

    // redlight_services ?? "specific" NPCs such as extra bouncer, wealthy gladiator, cardshark, healer ??

    let mut npc_text_set : String = "".into();
    for npc in npc_staff_list  {
        npc_text_set += &format!("({}) {} {}\n", npc.npc_type.to_string(),
        npc.species.to_string(),
        npc.task_description);
    }

    //---
    s.pop_layer();
    let app1 = app.clone();
    let app2 = app.clone();

    s.add_layer(
        Dialog::new()
            .title(dialog_title)
            .content(
                LinearLayout::horizontal()
                    .child(Dialog::text(gm_text).title("GM Notes").fixed_width(32))
                    .child(
                        Dialog::text(player_text)
                            .title("Player Notes")
                            .fixed_width(48)
                            .scrollable()
                            .scroll_y(true),
                    )
                    .child(
                        Dialog::text(npc_text_set)
                            .title("Notable Individuals")
                            .fixed_width(32)
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
