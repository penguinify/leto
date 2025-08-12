use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum IpcMessage {
    #[serde(rename = "drag_window")]
    DragWindow,
    #[serde(rename = "click_link")]
    ClickLink { url: String },

    #[serde(rename = "zoom")]
    Zoom { level: f64 },

    #[serde(rename = "loaded")]
    Loaded,
}
