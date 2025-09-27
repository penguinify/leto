use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn load_url_into_string(url: &str) -> Result<String, InjectionError> {
    tracing::info!("Loading script from URL: {}", url);
    match reqwest::blocking::get(url) {
        Ok(resp) => match resp.text() {
            Ok(text) => {
                tracing::info!("Successfully loaded script from URL: {}", url);
                Ok(text)
            }
            Err(e) => {
                tracing::error!("Failed to read script from URL '{}': {}", url, e);
                Err(InjectionError::ScriptLoadError(format!(
                    "Failed to read script from URL '{}': {}",
                    url, e
                )))
            }
        },
        Err(e) => {
            tracing::error!("Failed to fetch script from URL '{}': {}", url, e);
            Err(InjectionError::ScriptLoadError(format!(
                "Failed to fetch script from URL '{}': {}",
                url, e
            )))
        }
    }
}

pub fn load_script_into_string(script_name: &str) -> Result<String, InjectionError> {
    tracing::info!("Loading script: {}", script_name);

    if let Some(script_path) = get_script_path(script_name) {
        match fs::read_to_string(&script_path) {
            Ok(content) => {
                tracing::info!("Successfully loaded script from {}", script_path.display());
                Ok(content)
            }
            Err(e) => {
                tracing::error!("Failed to load script '{}': {}", script_path.display(), e);
                Err(InjectionError::ScriptLoadError(format!(
                    "Failed to load script '{}': {}",
                    script_path.display(),
                    e
                )))
            }
        }
    } else {
        tracing::error!(
            "Failed to determine executable directory; can't load script '{}'",
            script_name
        );
        Err(InjectionError::ScriptLoadError(format!(
            "Failed to determine executable directory for '{}'",
            script_name
        )))
    }
}

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

#[derive(Debug)]
pub enum InjectionError {
    ScriptLoadError(String),
    ScriptEvalError(String),
    ScriptInjectionError(String),
    UrlParseError(String),
    IoError(String),
}
