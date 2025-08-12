use std::fs;

const POST_INJECT_SCRIPTS_FILE: &str = "./scripts/post.js";
const PRE_INJECT_SCRIPTS_FILE: &str = "./scripts/pre.js";

pub fn load_script_into_string(script: &str) -> String {
    let mut script_string = String::new();
    if let Ok(content) = fs::read_to_string(script) {
        script_string.push_str(&content);
    }
    script_string
}

pub fn get_post_inject_script() -> String {
    load_script_into_string(POST_INJECT_SCRIPTS_FILE)
}

pub fn get_pre_inject_script() -> String {
    load_script_into_string(PRE_INJECT_SCRIPTS_FILE)
}
