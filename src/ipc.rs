use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum IpcMessage {
    #[serde(rename = "drag_window")]
    DragWindow,
    #[serde(rename = "click_link")]
    ClickLink {
        url: String,
    },

    #[serde(rename = "zoom_in")]
    ZoomIn,
    #[serde(rename = "zoom_out")]
    ZoomOut,

    #[serde(rename = "loaded")]
    Loaded
}
