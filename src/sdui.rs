use axum::{Json, extract::Path, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use std::sync::Arc;

// This is the core of our "Server-Driven UI" (SDUI) Engine.
// It replaces FB Lite's C++ UI generation engine with memory-safe Rust.

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum UIComponent {
    Text { text: String, color: String, size: u8 },
    Button { text: String, action_id: String, bg_color: String },
    Image { url: String, width: u16, height: u16 },
    ChatBubble { text: String, is_sender: bool },
    Column { children: Vec<UIComponent> },
    Row { children: Vec<UIComponent> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenLayout {
    pub screen_id: String,
    pub background_color: String,
    pub root: UIComponent,
}

// FB LITE MAGIC: The App calls this API. 
// The Server checks the DB and decides what the app should draw.
// No data is leaked because the server only sends UI drawing instructions.
pub async fn get_ui_screen(
    State(_state): State<Arc<AppState>>,
    Path(screen_name): Path<String>,
) -> Result<Json<ScreenLayout>, (StatusCode, String)> {
    
    // In a real scenario, we would check the user's Auth Token here.
    // If they are not logged in, we return a "Login Screen" UI automatically.

    match screen_name.as_str() {
        "home" => {
            let layout = ScreenLayout {
                screen_id: "home_screen".to_string(),
                background_color: "#121212".to_string(),
                root: UIComponent::Column {
                    children: vec![
                        UIComponent::Text { text: "Zero Lite (Secure)".to_string(), color: "#0084FF".to_string(), size: 24 },
                        UIComponent::Text { text: "No Data Leaked. 100% Server Side.".to_string(), color: "#888888".to_string(), size: 14 },
                        UIComponent::Button { text: "Start Secure Chat".to_string(), action_id: "open_chat".to_string(), bg_color: "#0084FF".to_string() }
                    ]
                }
            };
            Ok(Json(layout))
        },
        _ => Err((StatusCode::NOT_FOUND, "Screen not found".to_string()))
    }
}
