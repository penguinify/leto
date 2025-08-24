// I don't plan on finishing this implementation
// but you can plan aswell
// 
//  
// 

use rsrpc::{
    RPCConfig, RPCServer,
    detection::{DetectableActivity, Executable},
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::{thread, time::Duration};
use tracing::{error, info};
use wry::WebView;

static OBS_OPEN: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Deserialize)]
struct Payload {
    name: String,
    exe: String,
}

#[derive(Serialize, Deserialize)]
pub struct Window {
    title: String,
    process_name: String,
    pid: u32,
}

pub fn start_rpc_server() -> Result<Arc<Mutex<RPCServer>>, String> {
    #[cfg(debug_assertions)]
    unsafe {
        std::env::set_var("RSRPC_LOGS_ENABLED", "1");
    }
    let detectables =
        match reqwest::blocking::get("https://discord.com/api/v9/applications/detectable") {
            Ok(resp) => match resp.text() {
                Ok(text) => text,
                Err(e) => {
                    error!("Failed to read detectables: {}", e);
                    return Err("Failed to read detectables".to_string());
                }
            },
            Err(e) => {
                error!("Failed to fetch detectables: {}", e);
                return Err("Failed to fetch detectables".to_string());
            }
        };

    let rpc_config = RPCConfig {
        enable_process_scanner: true,
        enable_ipc_connector: true,
        enable_websocket_connector: true,
        enable_secondary_events: true,
    };

    let server = match RPCServer::from_json_str(detectables, rpc_config) {
        Ok(s) => Arc::new(Mutex::new(s)),
        Err(e) => {
            error!("Failed to start RPC server: {:?}", e);
            return Err(format!("Failed to start RPC server: {:?}", e));
        }
    };

    // Append local detectables (if you re-add file read/write functions)
    server
        .lock()
        .unwrap()
        .append_detectables(get_local_detectables());

    let server_clone = Arc::clone(&server);

    // Spawn the RPC loop
    thread::spawn(move || {
        server_clone.lock().unwrap().start();
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    });

    info!("RPC server started successfully.");

    Ok(server)
}

// Placeholder
fn get_local_detectables() -> Vec<DetectableActivity> {
    vec![]
}
