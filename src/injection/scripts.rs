use crate::injection::shared::load_script_into_string;

const SCRIPT_NAME_POST: &str = "post.js";

const SCRIPT_NAME_PRE: &str = "pre.js";


pub fn get_post_inject_script() -> String {
    tracing::info!("Getting post-inject script.");
    load_script_into_string(SCRIPT_NAME_POST).unwrap_or_default()
}

pub fn get_pre_inject_script() -> String {
    tracing::info!("Getting pre-inject script.");
    load_script_into_string(SCRIPT_NAME_PRE).unwrap_or_default()
}



