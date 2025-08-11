use std::fs;

const INTERNAL_SCRIPTS_DIR: &str = "./scripts/";
const SCRIPTS_FILE: &str = "./scripts/internal.js";

pub fn load_script_into_string(script: &str) -> String {
    let mut script_string = String::new();
    if let Ok(content) = fs::read_to_string(script) {
        script_string.push_str(&content);
    }
    println!("Loaded script: {}", script_string);
    script_string
}

pub fn get_internal_script() -> String {
    load_script_into_string(SCRIPTS_FILE)
}
