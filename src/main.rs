use crate::scripts::load_script_into_string;

mod ipc;
mod scripts;
mod window;

const TITLE: &str = "Leto";
const WEB_VIEW_URL: &str = "https://discord.com/app";
const INTERNAL_SCRIPTS_DIR: &str = "./scripts/";
const SCRIPTS_FILE: &str = "./scripts/internal.js";

fn main() {
    let mut app = window::App::new(TITLE, 800, 600, WEB_VIEW_URL);

    let scripts = load_script_into_string(SCRIPTS_FILE);
    app.evaluate_script(&scripts);

    app.add_menubar_items();

    app.run();
}
