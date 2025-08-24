use crate::scripts::get_post_inject_script;

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

    let mut app = window::App::new(TITLE, WEB_VIEW_URL);

    match app.evaluate_script(&get_post_inject_script()) {
        Ok(_) => info!("Successfully evaluated post-inject script."),
        Err(e) => error!("Failed to evaluate script: {}", e),
    }

    let client_mods = scripts::ClientMods {
        shelter: true,
        equicord: false,
        vencord: false,
        better_discord: false,
    }.get_scripts();

    for script in client_mods {
        let script_to_inject = if script.starts_with("http") {
            scripts::load_url_into_string(&script)
        } else {
            error!("Failed to load script '{}': Not a valid URL.", script);
            "".to_string()
        };

        if !script_to_inject.is_empty() {
            match app.evaluate_script(&script_to_inject) {
                Ok(_) => info!("Successfully injected client mod script: {}", script),
                Err(e) => error!("Failed to inject client mod script '{}': {}", script, e),
            }
        } else {
            error!("Script '{}' is empty or failed to load.", script);
        }
    }

    


    app.add_menubar_items();

    info!("Running application event loop.");
    app.run();
}
