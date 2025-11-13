use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum IpcMessage {
    #[serde(rename = "drag_window")]
    DragWindow,
    #[serde(rename = "zoom")]
    Zoom { level: f64 },
    #[serde(rename = "fetch")]
    Fetch {
        url: String,
        options: FetchOptions,
        req_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FetchOptions {
    pub method: String,
    pub headers: Option<serde_json::Value>,
    pub body: Option<String>,
}
