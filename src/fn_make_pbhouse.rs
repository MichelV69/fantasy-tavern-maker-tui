// ---- all the uses all the time
use cursive::Cursive;
use cursive::view::Resizable;
use cursive::view::Scrollable;
use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::TextView;

// --- my stuff ---
use dice_bag::tower::DiceResult;
use dice_bag::tower::RollDice;
use npc::build::NpcTypeCode;
use npc::build::Profile as npc_Profile;
use tavern::enums::list::EstablishmentQualityLevel;
use tavern::enums::list::SizeList;
use text_postproc::tpp::l1_heading;

use crate::dice_bag;
use crate::fn_save_pbhouse_to_file::save_pbhouse_to_file;
use crate::fn_view_npc_block::view_npc_block;
use crate::npc;
use crate::tavern;
use crate::tavern::structs::list::App;
use crate::tavern::structs::list::PBHouse;
use crate::text_postproc;

pub fn make_pbhouse(s: &mut Cursive, app: App) {
    let pbh = PBHouse::new();
    let mut gm_text: String = "".into();
    let mut player_text: String = "".into();

    //---
    let dialog_title = format!("P&B House: the {}", &pbh.name);
    for line in &pbh.general_info() {
        player_text += &format!("\n{}", line);
    }

    gm_text += &("\n\n".to_owned() + &l1_heading("Establishment History Notes".to_string()));
    for line in &pbh.establishment_history_notes {
        gm_text += &format!("\n{}", line);
    }

    gm_text += &("\n\n".to_owned() + &l1_heading("Redlight Services".to_string()));

    if pbh.redlight_services.iter().count() > 0 {
        for line in &pbh.redlight_services {
            gm_text += &format!("\n{}", line);
        }
    } else {
        gm_text += "\n\n_<none>_";
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
        npc_patron.set_random_encounter_chance_timeslots();
        npc_notable_patrons_list.push(npc_patron);
    }

    //todo!("redlight_services ?? \"specific\" NPCs such as extra bouncer, wealthy gladiator, cardshark, healer ??")
    use crate::npc::build::Profile;
    let mut npc_select = cursive::views::SelectView::new();

    let mut npc_full_list: Vec<Profile> = vec![];
    npc_full_list.append(&mut npc_staff_list);
    npc_full_list.append(&mut npc_notable_patrons_list);
    let npc_list1 = npc_full_list.clone();

    for npc in npc_full_list {
        let select_item = &format!(
            "({}) {} {}\n",
            npc.npc_type, npc.species, npc.task_description
        );
        npc_select.add_item(select_item, npc);
    }

    //---
    let app1 = app.clone();
    let app2 = app.clone();
    let pbh1 = pbh.clone();

    npc_select.set_on_submit(|s, npc| {
        let text = view_npc_block(npc);

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
                save_pbhouse_to_file(s, pbh1.clone(), app1.clone(), npc_list1.clone())
            })
            .button("Roll another", move |s| {
                s.pop_layer();
                make_pbhouse(s, app2.clone())
            })
            .button("Menu", |s| {
                s.pop_layer();
            })
            .h_align(cursive::align::HAlign::Center),
    );
}
// --- eof ---
