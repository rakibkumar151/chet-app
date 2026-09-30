use axum::{Json, extract::Path, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use std::sync::Arc;

// Server-Driven UI engine — FB Lite's "Bladerunner" in Rust

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum UIComponent {
    Text { text: String, color: String, size: u8 },
    Button { text: String, action_id: String, bg_color: String },
    Input { hint: String, field_key: String, input_type: String },
    Image { url: String, width: u16, height: u16 },
    ChatBubble { text: String, sender_name: String, is_sender: bool },
    Column { children: Vec<UIComponent> },
    Row { children: Vec<UIComponent> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenLayout {
    pub screen_id: String,
    pub background_color: String,
    pub root: UIComponent,
}

pub async fn get_ui_screen(
    State(_state): State<Arc<AppState>>,
    Path(screen_name): Path<String>,
) -> Result<Json<ScreenLayout>, (StatusCode, String)> {

    match screen_name.as_str() {
        // ── SIGNUP SCREEN ──────────────────────────────────────────────
        "signup" => Ok(Json(ScreenLayout {
            screen_id: "signup_screen".to_string(),
            background_color: "#121212".to_string(),
            root: UIComponent::Column {
                children: vec![
                    UIComponent::Text { text: "Zero Lite".to_string(), color: "#0084FF".to_string(), size: 28 },
                    UIComponent::Text { text: "Create your account".to_string(), color: "#888888".to_string(), size: 14 },
                    UIComponent::Input { hint: "First Name".to_string(), field_key: "first_name".to_string(), input_type: "text".to_string() },
                    UIComponent::Input { hint: "Last Name".to_string(), field_key: "last_name".to_string(), input_type: "text".to_string() },
                    UIComponent::Input { hint: "Email Address".to_string(), field_key: "email".to_string(), input_type: "email".to_string() },
                    UIComponent::Input { hint: "Gender (Male/Female/Other)".to_string(), field_key: "gender".to_string(), input_type: "text".to_string() },
                    UIComponent::Button { text: "Create Account".to_string(), action_id: "do_signup".to_string(), bg_color: "#0084FF".to_string() },
                    UIComponent::Button { text: "Already have an account? Login".to_string(), action_id: "go_login".to_string(), bg_color: "#3A3B3C".to_string() },
                ]
            }
        })),

        // ── LOGIN SCREEN ───────────────────────────────────────────────
        "login" => Ok(Json(ScreenLayout {
            screen_id: "login_screen".to_string(),
            background_color: "#121212".to_string(),
            root: UIComponent::Column {
                children: vec![
                    UIComponent::Text { text: "Zero Lite".to_string(), color: "#0084FF".to_string(), size: 28 },
                    UIComponent::Text { text: "Login to your account".to_string(), color: "#888888".to_string(), size: 14 },
                    UIComponent::Input { hint: "Email Address".to_string(), field_key: "email".to_string(), input_type: "email".to_string() },
                    UIComponent::Button { text: "Login".to_string(), action_id: "do_login".to_string(), bg_color: "#0084FF".to_string() },
                    UIComponent::Button { text: "New here? Create Account".to_string(), action_id: "go_signup".to_string(), bg_color: "#3A3B3C".to_string() },
                ]
            }
        })),

        // ── USERS LIST (PEOPLE TO CHAT) ────────────────────────────────
        "users" => Ok(Json(ScreenLayout {
            screen_id: "users_screen".to_string(),
            background_color: "#121212".to_string(),
            root: UIComponent::Column {
                children: vec![
                    UIComponent::Text { text: "Zero Chat".to_string(), color: "#0084FF".to_string(), size: 22 },
                    UIComponent::Text { text: "People".to_string(), color: "#FFFFFF".to_string(), size: 18 },
                    UIComponent::Button { text: "Refresh".to_string(), action_id: "load_users".to_string(), bg_color: "#3A3B3C".to_string() },
                ]
            }
        })),

        // ── HOME ───────────────────────────────────────────────────────
        "home" => Ok(Json(ScreenLayout {
            screen_id: "home_screen".to_string(),
            background_color: "#121212".to_string(),
            root: UIComponent::Column {
                children: vec![
                    UIComponent::Text { text: "Zero Lite".to_string(), color: "#0084FF".to_string(), size: 28 },
                    UIComponent::Text { text: "Secure messaging. Zero data leaks.".to_string(), color: "#888888".to_string(), size: 14 },
                    UIComponent::Button { text: "Get Started".to_string(), action_id: "go_signup".to_string(), bg_color: "#0084FF".to_string() },
                    UIComponent::Button { text: "Login".to_string(), action_id: "go_login".to_string(), bg_color: "#3A3B3C".to_string() },
                ]
            }
        })),

        _ => Err((StatusCode::NOT_FOUND, "Screen not found".to_string()))
    }
}
