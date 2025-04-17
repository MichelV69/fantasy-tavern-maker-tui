use cursive::views::TextView;
// ---- all the uses all the time
use cursive::Cursive;
use cursive::view::Resizable;
use cursive::view::Scrollable;
use cursive::views::Dialog;
use cursive::views::LinearLayout;
use indoc::formatdoc;
use std::fs::File;
use std::io::prelude::*;

// --- my stuff ---
mod dice_bag;
mod npc;
mod tavern;
mod text_postproc;

use dice_bag::tower::DiceResult;
use dice_bag::tower::RollDice;
use dirs::download_dir;
use npc::build::NpcTypeCode;
use npc::build::Profile as npc_Profile;
use tavern::enums::list::EstablishmentQualityLevel;
use tavern::enums::list::SizeList;
use tavern::structs::list::App;
use tavern::structs::list::PBHouse;
use tavern::traits::list::AppFn;
use text_postproc::tpp::l1_heading;

// --- local cli code
fn main() {
    let mut app: App = App::new();
    app.name = "fantasy-tavern-maker-tui".into();
    app.version_major = 0;
    app.version_minor = 10;
    app.version_fix = 1;
    app.version_build = 138;

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

    let roll_string: String = format!("{}{}{}", die_size, math_func.to_string(), roll_mod);
    //println!("roll_string [{roll_string}]");

    let npc_notable_patrons_count: i16 =
        <DiceResult as RollDice>::from_string(&roll_string).get_total();
    //println!("npc_notable_patrons_count [{npc_notable_patrons_count}]");

    for _c in 1..npc_notable_patrons_count {
        let mut npc_patron: npc_Profile = npc_Profile::new();
        npc_patron.npc_type = NpcTypeCode::Patron;
        npc_patron.set_random_task_description(&app);
        npc_patron.random_appearance(&app);
        npc_patron.set_random_quirk_emotional(&app);
        npc_patron.set_random_schticks_attributes(&app);
        npc_notable_patrons_list.push(npc_patron);
    }

    //todo!("redlight_services ?? \"specific\" NPCs such as extra bouncer, wealthy gladiator, cardshark, healer ??")
    use crate::npc::build::Profile;
    let mut npc_select = cursive::views::SelectView::new();

    let mut npc_full_list: Vec<Profile> = vec![];
    npc_full_list.append(&mut npc_staff_list);
    npc_full_list.append(&mut npc_notable_patrons_list);
    for npc in npc_full_list {
        let select_item = &format!(
            "({}) {} {}\n",
            npc.npc_type, npc.species, npc.task_description
        );
        npc_select.add_item(select_item, npc);
    }

    //---
    s.pop_layer();
    let app1 = app.clone();
    let app2 = app.clone();
    let pbh1 = pbh.clone();

    npc_select.set_on_submit(|s, npc| {
        let text = formatdoc!(
            r#"
            {npc_type} {task}

            {height_desc} {build_desc} {gender:?} {species:?}
            with {eye_color:?} eyes, and {hair_color} hair in {hair_style} style

            Quirks:
                + {quirk_emotional}
                + {quirk_physical}
            Notable Attributes:
                + {notable_attribute_positive}
                + {notable_attribute_negative}

        "#,
            npc_type = npc.npc_type,
            task = npc.task_description,
            eye_color = npc.eye_color,
            height_desc = npc.height_desc,
            build_desc = npc.build_desc,
            gender = npc.gender,
            species = npc.species,
            hair_color = npc.hair_color.to_string(),
            hair_style = npc.hair_style.to_string(),
            quirk_emotional = npc.quirk_emotional.to_string(),
            quirk_physical = npc.quirk_physical.to_string(),
            notable_attribute_positive = npc.notable_attribute_positive.to_string(),
            notable_attribute_negative = npc.notable_attribute_negative.to_string()
        );

        s.add_layer(Dialog::around(TextView::new(text)).button("Done", |s| {
            s.pop_layer();
        }));
    });

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
                        Dialog::around(npc_select)
                            .title("Notable Individuals")
                            .fixed_width(32)
                            .scrollable()
                            .scroll_y(true),
                    ),
            )
            .button("Save to file", move |s| {
                save_pbhouse_to_file(s, pbh1.clone(), app1.clone())
            })
            .button("Roll another", move |s| get_new_pbhouse(s, app2.clone()))
            .button("Quit", |s| s.quit())
            .h_align(cursive::align::HAlign::Center),
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
