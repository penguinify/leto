use std::fs;

pub fn load_script_into_string(script: &str) -> String {
    let mut script_string = String::new();
    if let Ok(content) = fs::read_to_string(script) {
        script_string.push_str(&content);
    }
    println!("Loaded script: {}", script_string);
    script_string
}
