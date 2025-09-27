mod injection;

mod ipc;
mod rpc;
mod window;

use injection::scripts;

const TITLE: &str = "Leto";
const WEB_VIEW_URL: &str = "https://discord.com/app";

use tracing::{error, info};

fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting application: {}", TITLE);

    let mut app = window::App::new(TITLE, WEB_VIEW_URL);

    match app.evaluate_script(&scripts::get_post_inject_script()) {
        Ok(_) => info!("Successfully evaluated post-inject script."),
        Err(e) => error!("Failed to evaluate script: {}", e),
    }

    let mut client_mods = injection::client_mods::ClientMods::new();

    client_mods.add_mod(injection::client_mods::ClientMod {
        name: "Shelter".to_string(),
        script: "https://raw.githubusercontent.com/uwu/shelter-builds/main/shelter.js".to_string(),
        styles: None,
    });

    let injectables = client_mods.into_injectables();
    for injectable in injectables {
        match injection::inject::inject_injectable(&mut app, injectable) {
            Ok(_) => info!("Successfully injected client mod."),
            Err(_e) => error!("Failed to inject client mod"),
        }
    }

    app.add_menubar_items();

    info!("Running application event loop.");
    app.run();
}
