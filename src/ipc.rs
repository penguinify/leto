use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum IpcMessage {
    #[serde(rename = "drag_window")]
    DragWindow,

    #[serde(rename = "zoom")]
    Zoom { level: f64 },
}
