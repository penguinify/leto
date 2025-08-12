use crate::scripts::get_post_inject_script;

mod ipc;
mod scripts;
mod window;

const TITLE: &str = "Leto";
const WEB_VIEW_URL: &str = "https://discord.com/app";

fn main() {
    let mut app = window::App::new(TITLE, 800, 600, WEB_VIEW_URL);

    app.evaluate_script(&get_post_inject_script()).unwrap_or_else(|e| {
        eprintln!("Failed to evaluate script: {}", e);
    });

    app.add_menubar_items();

    app.run();
}
