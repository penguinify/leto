use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const SCRIPT_NAME_POST: &str = "post.js";

const SCRIPT_NAME_PRE: &str = "pre.js";

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

pub fn load_url_into_string(url: &str) -> String {
    tracing::info!("Loading script from URL: {}", url);
    match reqwest::blocking::get(url) {
        Ok(resp) => match resp.text() {
            Ok(text) => {
                tracing::info!("Successfully loaded script from URL: {}", url);
                text
            }
            Err(e) => {
                tracing::error!("Failed to read script from URL '{}': {}", url, e);
                String::new()
            }
        },
        Err(e) => {
            tracing::error!("Failed to fetch script from URL '{}': {}", url, e);
            String::new()
        }
    }
}

pub fn get_post_inject_script() -> String {
    tracing::info!("Getting post-inject script.");
    load_script_into_string(SCRIPT_NAME_POST)
}

pub fn get_pre_inject_script() -> String {
    tracing::info!("Getting pre-inject script.");
    load_script_into_string(SCRIPT_NAME_PRE)
}

#[derive(Debug, Default)]
pub struct ClientMods {
    pub vencord: bool,
    pub better_discord: bool,
    pub shelter: bool, // Required as it hosts some scripts for Leto
    pub equicord: bool,
}


impl ClientMods {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, name: &str, value: bool) {
        match name {
            "vencord" => self.vencord = value,
            "better_discord" => self.better_discord = value,
            "shelter" => self.shelter = value,
            "equicord" => self.equicord = value,
            _ => (),
        }
    }

    pub fn get_scripts(&self) -> Vec<String> {
        let mut scripts = Vec::new();

        if self.vencord {
            scripts.push("https://github.com/Vendicated/Vencord/releases/download/devbuild/vencordDesktopMain.js".to_string());
        }
        if self.better_discord {
            scripts.push("better_discord.js".to_string());
        }
        if self.shelter {
            scripts.push("https://raw.githubusercontent.com/uwu/shelter-builds/main/shelter.js".to_string());
        }
        if self.equicord {
            scripts.push("equicord.js".to_string());
        }

        scripts
    }
}
