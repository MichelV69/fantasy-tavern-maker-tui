// ---- enabled features for the project
#![feature(variant_count)]

use cursive::event::Key;
// ---- all the uses all the time
use cursive::views::Dialog;

// --- my stuff ---
mod dice_bag;
mod fn_make_pbhouse;
mod fn_save_pbhouse_to_file;
mod fn_view_npc_block;
mod narrative_time_manager;
mod npc;
mod tavern;
mod text_postproc;

use fn_make_pbhouse::make_pbhouse;
use tavern::structs::list::App;
use tavern::traits::list::AppFn;

// --- local cli code
fn main() {
    let mut app: App = App::new();
    app.name = "fantasy-tavern-maker-tui".into();
    app.version_major = 0; //0.13.2-195
    app.version_minor = 13;
    app.version_fix = 02;
    app.version_build = 195;

    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text(format!("Welcome to {} ({})", &app.name, &app.get_version()))
            .title(&app.name)
            .button("Create a new P&B House?", move |s| {
                make_pbhouse(s, app.clone())
            })
            .button("Quit", |s| s.quit()),
    );
    siv.add_global_callback(Key::Esc, |s| {
        s.pop_layer();
    });

    siv.run()
}
// --- eof ---
