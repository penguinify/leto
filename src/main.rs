use crate::scripts::get_post_inject_script;

mod html_patching;
mod ipc;
mod microphone;
mod rpc;
mod scripts;
mod window;

const TITLE: &str = "Leto";
const WEB_VIEW_URL: &str = "https://discord.com/app";

use tracing::{error, info};

fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting application: {}", TITLE);

    let mut app = window::App::new(TITLE, 800, 600, WEB_VIEW_URL);

    match app.evaluate_script(&get_post_inject_script()) {
        Ok(_) => info!("Successfully evaluated post-inject script."),
        Err(e) => error!("Failed to evaluate script: {}", e),
    }

    app.add_menubar_items();

    info!("Running application event loop.");
    app.run();
}
