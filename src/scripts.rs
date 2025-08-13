use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn get_executable_dir() -> Option<PathBuf> {
    env::current_exe()
        .ok()
        .and_then(|exe_path| exe_path.parent().map(|p| p.to_path_buf()))
}

fn get_script_path(script_name: &str) -> Option<PathBuf> {
    if cfg!(debug_assertions) {
        env::current_dir()
            .ok()
            .map(|cwd| cwd.join("scripts").join(script_name))
    } else if cfg!(target_os = "macos") {
        get_executable_dir().map(|exe_dir| exe_dir.join("../Resources/scripts").join(script_name))
    } else {
        get_executable_dir().map(|exe_dir| exe_dir.join("scripts").join(script_name))
    }
}

pub fn load_script_into_string(script_name: &str) -> String {
    tracing::info!("Loading script: {}", script_name);

    if let Some(script_path) = get_script_path(script_name) {
        match fs::read_to_string(&script_path) {
            Ok(content) => {
                tracing::info!("Successfully loaded script from {}", script_path.display());
                content
            }
            Err(e) => {
                tracing::error!("Failed to load script '{}': {}", script_path.display(), e);
                String::new()
            }
        }
    } else {
        tracing::error!(
            "Failed to determine executable directory; can't load script '{}'",
            script_name
        );
        String::new()
    }
}

pub fn get_post_inject_script() -> String {
    tracing::info!("Getting post-inject script.");
    load_script_into_string("post.js")
}

pub fn get_pre_inject_script() -> String {
    tracing::info!("Getting pre-inject script.");
    load_script_into_string("pre.js")
}
