use axum::{Json, extract::Path, extract::Query, http::StatusCode, http::HeaderMap, extract::State};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use crate::handlers::{extract_token, verify_token};
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum UIComponent {
    Text { text: String, #[serde(skip_serializing_if = "Option::is_none")] action_id: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] size: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] align: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] bold: Option<bool>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    CircleText { text: String, #[serde(skip_serializing_if = "Option::is_none")] action_id: Option<String>, color: String, bg_color: String, #[serde(skip_serializing_if = "Option::is_none")] border_color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] size: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] gravity: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8> },
    Input { hint: String, field_key: String, input_type: String, #[serde(skip_serializing_if = "Option::is_none")] action_id: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    Button { text: String, action_id: String, bg_color: String, #[serde(skip_serializing_if = "Option::is_none")] text_color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    ButtonOutline { text: String, action_id: String, border_color: String, #[serde(skip_serializing_if = "Option::is_none")] text_color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    Divider { #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8> },
    Spacer { height: u8 },
    ChatBubble { text: String, is_mine: bool, #[serde(skip_serializing_if = "Option::is_none")] sender_initial: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] recipient_initial: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] status_text: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] time_text: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8> },
    Column { children: Vec<UIComponent>, #[serde(skip_serializing_if = "Option::is_none")] action_id: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] align: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    Row { children: Vec<UIComponent>, #[serde(skip_serializing_if = "Option::is_none")] action_id: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] bg_color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] gravity: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    HorizontalScroll { children: Vec<UIComponent>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    Stack { children: Vec<UIComponent>, #[serde(skip_serializing_if = "Option::is_none")] gravity: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenLayout {
    pub screen_id: String,
    pub background_color: String,
    pub root: UIComponent,
}

fn format_active_status(is_online: bool, last_seen_str: &str) -> (String, String) {
    if is_online {
        return ("Active now".to_string(), "#31A24C".to_string());
    }
    let status_text = chrono::NaiveDateTime::parse_from_str(last_seen_str, "%Y-%m-%d %H:%M:%S")
        .ok()
        .map(|t| {
            let now = chrono::Utc::now().naive_utc();
            let secs = (now - t).num_seconds().max(0);
            if secs < 10 {
                "Active just now".to_string()
            } else if secs < 60 {
                format!("Active {}s ago", secs)
            } else if secs < 3600 {
                format!("Active {}m ago", secs / 60)
            } else if secs < 86400 {
                format!("Active {}h ago", secs / 3600)
            } else if secs < 2592000 {
                format!("Active {}d ago", secs / 86400)
            } else if secs < 31536000 {
                format!("Active {}mo ago", secs / 2592000)
            } else {
                format!("Active {}y ago", secs / 31536000)
            }
        })
        .unwrap_or_else(|| "Offline".to_string());
    (status_text, "#90949C".to_string())
}

pub async fn get_ui_screen(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(screen_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ScreenLayout>, (StatusCode, String)> {

    let online_uids: std::collections::HashSet<String> = {
        let clients = state.clients.read().await;
        clients.get("zero_lite")
            .map(|map| map.keys().cloned().collect())
            .unwrap_or_default()
    };
    let is_connected_ws = |target_uid: &str, db_online: bool| -> bool {
        db_online || online_uids.contains(target_uid)
    };

    let parts: Vec<&str> = screen_name.split('/').collect();
    let name = parts[0];

    match name {
        // ── LOGIN SCREEN (FB LITE STYLE) ──────────────────────────────
        "login" => Ok(Json(ScreenLayout {
            screen_id: "login_screen".to_string(),
            background_color: "#FFFFFF".to_string(),
            root: UIComponent::Column {
                action_id: None,
                align: Some("center".to_string()),
                padding_top: Some(40), padding_bottom: Some(32), padding_start: Some(32), padding_end: Some(32),
                margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None,
                children: vec![
                    UIComponent::Text { text: "English (US)  ⌄".to_string(), action_id: None, color: Some("#606770".to_string()), size: Some(13), align: Some("end".to_string()), bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None },
                    UIComponent::CircleText { text: "f".to_string(), action_id: None, color: "#1877F2".to_string(), bg_color: "#FFFFFF".to_string(), border_color: Some("#DADDE1".to_string()), size: Some(72), gravity: None, margin_top: Some(16), margin_bottom: Some(8), margin_start: None, margin_end: None },
                    UIComponent::Spacer { height: 24 },
                    UIComponent::Input { hint: "Mobile number or email".to_string(), field_key: "email".to_string(), input_type: "email".to_string(), action_id: None, margin_top: None, margin_bottom: Some(12), margin_start: None, margin_end: None, weight: None },
                    UIComponent::Input { hint: "Password".to_string(), field_key: "password".to_string(), input_type: "password".to_string(), action_id: None, margin_top: None, margin_bottom: Some(16), margin_start: None, margin_end: None, weight: None },
                    UIComponent::Button { text: "Log in".to_string(), action_id: "do_login".to_string(), bg_color: "#1877F2".to_string(), text_color: Some("#FFFFFF".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None },
                    UIComponent::Text { text: "Forgot password?".to_string(), action_id: None, color: Some("#1877F2".to_string()), size: Some(14), align: Some("center".to_string()), bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(16), padding_bottom: Some(24), padding_start: None, padding_end: None, weight: None },
                    UIComponent::Divider { margin_top: None, margin_bottom: None },
                    UIComponent::Spacer { height: 20 },
                    UIComponent::ButtonOutline { text: "Create new account".to_string(), action_id: "go_signup".to_string(), border_color: "#1877F2".to_string(), text_color: Some("#1877F2".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None },
                    UIComponent::Spacer { height: 32 },
                    UIComponent::Text { text: "Meta ∞".to_string(), action_id: None, color: Some("#606770".to_string()), size: Some(13), align: Some("center".to_string()), bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                ]
            }
        })),

        // ── SIGNUP SCREEN ──────────────────────────────────────────────
        "signup" => Ok(Json(ScreenLayout {
            screen_id: "signup_screen".to_string(),
            background_color: "#FFFFFF".to_string(),
            root: UIComponent::Column {
                action_id: None,
                align: Some("center".to_string()),
                padding_top: Some(16), padding_bottom: Some(32), padding_start: Some(24), padding_end: Some(24),
                margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None,
                children: vec![
                    // Back Arrow
                    UIComponent::Row {
                        action_id: None, bg_color: None, gravity: None, margin_top: None, margin_bottom: Some(16), margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                        children: vec![
                            UIComponent::Text { text: "←".to_string(), action_id: Some("go_login".to_string()), color: Some("#1C1E21".to_string()), size: Some(24), align: None, bold: Some(true), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                        ]
                    },
                    // Header Text
                    UIComponent::Text { text: "Create a new account".to_string(), action_id: None, color: Some("#1C1E21".to_string()), size: Some(22), align: Some("center".to_string()), bold: Some(true), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None },
                    UIComponent::Text { text: "It's quick and easy.".to_string(), action_id: None, color: Some("#606770".to_string()), size: Some(14), align: Some("center".to_string()), bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(4), padding_bottom: Some(20), padding_start: None, padding_end: None, weight: None },
                    UIComponent::Divider { margin_top: None, margin_bottom: None },
                    UIComponent::Spacer { height: 16 },
                    UIComponent::Row {
                        action_id: None, bg_color: None, gravity: None, margin_top: None, margin_bottom: Some(12), margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                        children: vec![
                            UIComponent::Input { hint: "First name".to_string(), field_key: "first_name".to_string(), input_type: "text".to_string(), action_id: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(8), weight: Some(1.0) },
                            UIComponent::Input { hint: "Last name".to_string(), field_key: "last_name".to_string(), input_type: "text".to_string(), action_id: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: Some(1.0) }
                        ]
                    },
                    UIComponent::Input { hint: "Mobile number or email".to_string(), field_key: "email".to_string(), input_type: "email".to_string(), action_id: None, margin_top: None, margin_bottom: Some(12), margin_start: None, margin_end: None, weight: None },
                    UIComponent::Input { hint: "Gender (Male/Female)".to_string(), field_key: "gender".to_string(), input_type: "text".to_string(), action_id: None, margin_top: None, margin_bottom: Some(12), margin_start: None, margin_end: None, weight: None },
                    UIComponent::Input { hint: "New password".to_string(), field_key: "password".to_string(), input_type: "password".to_string(), action_id: None, margin_top: None, margin_bottom: Some(20), margin_start: None, margin_end: None, weight: None },
                    UIComponent::Button { text: "Sign Up".to_string(), action_id: "do_signup".to_string(), bg_color: "#1877F2".to_string(), text_color: Some("#FFFFFF".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None },
                    UIComponent::Spacer { height: 12 },
                    UIComponent::Text { text: "Already have an account?".to_string(), action_id: None, color: Some("#1877F2".to_string()), size: Some(14), align: Some("center".to_string()), bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                ]
            }
        })),

        // ── USERS LIST ─────────────────────────────────────────────────
        "users" => {
            let mut children = vec![
                UIComponent::Row {
                    action_id: None, bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(14), padding_bottom: Some(14), padding_start: Some(16), padding_end: Some(16), weight: None,
                    children: vec![
                        UIComponent::Text { text: "Zero Chat".to_string(), action_id: None, color: Some("#1877F2".to_string()), size: Some(20), align: None, bold: Some(true), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0) },
                        UIComponent::Button { text: "↻".to_string(), action_id: "refresh_users".to_string(), bg_color: "#FFFFFF".to_string(), text_color: Some("#1877F2".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(8), weight: None },
                        UIComponent::Button { text: "⏻".to_string(), action_id: "do_logout".to_string(), bg_color: "#FFFFFF".to_string(), text_color: Some("#90949C".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None }
                    ]
                }
            ];

            // Fetch users from DB
            let conn = state.db.db.connect().unwrap();
            if let Ok(mut rows) = conn.query("SELECT uid, first_name, last_name, is_online FROM users", ())
                .await {
                while let Ok(Some(row)) = rows.next().await {
                    let uid: String = row.get(0).unwrap_or_default();
                    let fn_name: String = row.get(1).unwrap_or_default();
                    let ln_name: String = row.get(2).unwrap_or_default();
                    let is_online = row.get::<i64>(3).unwrap_or(0) == 1;

                    let initial = fn_name.chars().next().unwrap_or('?').to_string().to_uppercase();
                    let status = if is_online { "Active now" } else { "Offline" };
                    let status_color = if is_online { "#31A24C" } else { "#90949C" };

                    children.push(
                        UIComponent::Row {
                            action_id: Some(format!("chat_{}", uid)), bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: Some(1), margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(12), padding_bottom: Some(12), padding_start: Some(16), padding_end: Some(16), weight: None,
                            children: vec![
                                UIComponent::CircleText { text: initial, action_id: None, color: "#FFFFFF".to_string(), bg_color: "#1877F2".to_string(), border_color: None, size: Some(48), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(12) },
                                UIComponent::Column {
                                    action_id: None,
                                    align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0),
                                    children: vec![
                                        UIComponent::Text { text: format!("{} {}", fn_name, ln_name), action_id: None, color: Some("#1C1E21".to_string()), size: Some(15), align: None, bold: Some(true), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None },
                                        UIComponent::Text { text: status.to_string(), action_id: None, color: Some(status_color.to_string()), size: Some(12), align: None, bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                                    ]
                                },
                                UIComponent::ButtonOutline { text: "Chat".to_string(), action_id: format!("chat_{}", uid), border_color: "#E4E6EB".to_string(), text_color: Some("#1C1E21".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None }
                            ]
                        }
                    );
                }
            }

            Ok(Json(ScreenLayout {
                screen_id: "users_screen".to_string(),
                background_color: "#F0F2F5".to_string(),
                root: UIComponent::Column {
                    action_id: None,
                    align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                    children
                }
            }))
        }

        // ── HOME / CHATS SCREEN ───────────────────────────────────────
        "home" => {
            let my_uid_param = params.get("my_uid").map(|s| s.as_str()).unwrap_or("").to_string();
            let token_header = extract_token(&headers).ok();

            let conn = state.db.db.connect().unwrap();

            // Resolve user identity (uid, initial, full_name) in DB
            let (my_uid, my_initial, my_full_name) = {
                let mut f_uid = my_uid_param.clone();
                let mut f_first = String::new();
                let mut f_last = String::new();
                let mut f_email = String::new();

                // 1. Try DB lookup by my_uid_param
                if !f_uid.is_empty() {
                    if let Ok(mut rows) = conn.query(
                        "SELECT uid, first_name, last_name, email FROM users WHERE uid = ?1",
                        libsql::params![f_uid.clone()]
                    ).await {
                        if let Ok(Some(row)) = rows.next().await {
                            f_uid = row.get(0).unwrap_or_default();
                            f_first = row.get(1).unwrap_or_default();
                            f_last = row.get(2).unwrap_or_default();
                            f_email = row.get(3).unwrap_or_default();
                        }
                    }
                }

                // 2. If first/last name not found by my_uid_param, try by Bearer token
                if f_first.is_empty() && f_last.is_empty() {
                    if let Some(ref tok) = token_header {
                        if let Ok(mut rows) = conn.query(
                            "SELECT uid, first_name, last_name, email FROM users WHERE token = ?1",
                            libsql::params![tok.clone()]
                        ).await {
                            if let Ok(Some(row)) = rows.next().await {
                                f_uid = row.get(0).unwrap_or_default();
                                f_first = row.get(1).unwrap_or_default();
                                f_last = row.get(2).unwrap_or_default();
                                f_email = row.get(3).unwrap_or_default();
                            }
                        }
                    }
                }

                // Construct full name
                let mut full_name = format!("{} {}", f_first, f_last).trim().to_string();
                if full_name.is_empty() || full_name == "User" {
                    if !f_email.is_empty() {
                        full_name = f_email.split('@').next().unwrap_or("User").to_string();
                    } else if !f_uid.is_empty() {
                        let short = if f_uid.len() > 8 { &f_uid[..8] } else { &f_uid };
                        full_name = format!("User {}", short);
                    } else {
                        full_name = "Active User".to_string();
                    }
                }

                // Construct initial letter
                let initial = full_name.chars().next().unwrap_or('M').to_string().to_uppercase();

                (f_uid, initial, full_name)
            };

            let mut children = vec![
                // 1. Top Bar
                UIComponent::Row {
                    action_id: None, bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(12), padding_bottom: Some(12), padding_start: Some(16), padding_end: Some(16), weight: None,
                    children: vec![
                        UIComponent::CircleText { text: my_initial.clone(), action_id: None, color: "#FFFFFF".to_string(), bg_color: "#1877F2".to_string(), border_color: None, size: Some(36), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(12) },
                        UIComponent::Column {
                            action_id: None,
                            align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0),
                            children: vec![
                                UIComponent::Text { text: "Chats".to_string(), action_id: None, color: Some("#1C1E21".to_string()), size: Some(20), align: None, bold: Some(true), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None },
                                UIComponent::Text { text: my_full_name.clone(), action_id: None, color: Some("#65676B".to_string()), size: Some(13), align: None, bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                            ]
                        },
                        UIComponent::CircleText { text: "📷".to_string(), action_id: None, color: "#1C1E21".to_string(), bg_color: "#F0F2F5".to_string(), border_color: None, size: Some(36), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(12) },
                        UIComponent::CircleText { text: "✏".to_string(), action_id: None, color: "#1C1E21".to_string(), bg_color: "#F0F2F5".to_string(), border_color: None, size: Some(36), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None },
                    ]
                },
                // 2. Search Bar
                UIComponent::Row {
                    action_id: None, bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(4), padding_bottom: Some(12), padding_start: Some(16), padding_end: Some(16), weight: None,
                    children: vec![
                        UIComponent::Input { hint: "🔍 Search".to_string(), field_key: "search".to_string(), input_type: "text".to_string(), action_id: Some("search_users".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: Some(1.0) }
                    ]
                }
            ];

            // 3. Active Users (Story Tray)
            let mut active_tray = vec![];
            
            active_tray.push(
                UIComponent::Column {
                    action_id: None,
                    align: Some("center".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(16), padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                    children: vec![
                        UIComponent::Stack {
                            gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                            children: vec![
                                UIComponent::CircleText { text: my_initial.clone(), action_id: None, color: "#FFFFFF".to_string(), bg_color: "#1877F2".to_string(), border_color: Some("#E4E6EB".to_string()), size: Some(56), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None },
                                UIComponent::CircleText { text: "+".to_string(), action_id: None, color: "#1C1E21".to_string(), bg_color: "#FFFFFF".to_string(), border_color: Some("#E4E6EB".to_string()), size: Some(18), gravity: Some("bottom_end".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None }
                            ]
                        },
                        UIComponent::Text { text: "Your story".to_string(), action_id: None, color: Some("#1C1E21".to_string()), size: Some(12), align: Some("center".to_string()), bold: None, margin_top: Some(6), margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                    ]
                }
            );

            let story_sql = "SELECT uid, first_name, is_online FROM users WHERE uid != ?1 ORDER BY is_online DESC, uid ASC LIMIT 10";
            if let Ok(mut rows) = conn.query(story_sql, libsql::params![my_uid.clone()]).await {
                while let Ok(Some(row)) = rows.next().await {
                    let uid: String = row.get(0).unwrap_or_default();
                    if uid == my_uid { continue; }
                    let fn_name: String = row.get(1).unwrap_or_default();
                    let db_online = row.get::<i64>(2).unwrap_or(0) == 1;
                    let is_online = is_connected_ws(&uid, db_online);
                    let initial = fn_name.chars().next().unwrap_or('?').to_string().to_uppercase();
                    
                    let ring_color = if is_online { Some("#1877F2".to_string()) } else { None };
                    
                    let mut stack_children = vec![
                        UIComponent::CircleText { text: initial, action_id: None, color: "#FFFFFF".to_string(), bg_color: "#A0A0A0".to_string(), border_color: ring_color, size: Some(56), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None }
                    ];
                    if is_online {
                        stack_children.push(UIComponent::CircleText { text: "".to_string(), action_id: None, color: "#FFFFFF".to_string(), bg_color: "#31A24C".to_string(), border_color: Some("#FFFFFF".to_string()), size: Some(16), gravity: Some("bottom_end".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None });
                    }
                    
                    active_tray.push(
                        UIComponent::Column {
                            action_id: Some(format!("chat_{}", uid)),
                            align: Some("center".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(16), padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                            children: vec![
                                UIComponent::Stack {
                                    gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                                    children: stack_children
                                },
                                UIComponent::Text { text: fn_name, action_id: None, color: Some("#1C1E21".to_string()), size: Some(12), align: Some("center".to_string()), bold: None, margin_top: Some(6), margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                            ]
                        }
                    );
                }
            }

            children.push(UIComponent::HorizontalScroll {
                margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(8), padding_bottom: Some(16), padding_start: Some(16), padding_end: Some(16), weight: None,
                children: active_tray
            });

            // 4. Chat List
            let search_query = params.get("q").map(|s| s.as_str()).unwrap_or("");
            let q_param = format!("%{}%", search_query);
            
            let query_res = if search_query.is_empty() {
                conn.query("SELECT uid, first_name, last_name, is_online, last_seen FROM users WHERE uid != ?1 ORDER BY is_online DESC, uid ASC", libsql::params![my_uid.clone()]).await
            } else {
                conn.query("SELECT uid, first_name, last_name, is_online, last_seen FROM users WHERE uid != ?1 AND (first_name LIKE ?2 OR last_name LIKE ?2) ORDER BY is_online DESC, uid ASC", libsql::params![my_uid.clone(), q_param.clone()]).await
            };
            
            if let Ok(mut rows) = query_res {
                let mut count = 0;
                while let Ok(Some(row)) = rows.next().await {
                    count += 1;
                    let uid: String = row.get(0).unwrap_or_default();
                    let fn_name: String = row.get(1).unwrap_or_default();
                    let ln_name: String = row.get(2).unwrap_or_default();
                    let db_online = row.get::<i64>(3).unwrap_or(0) == 1;
                    let is_online = is_connected_ws(&uid, db_online);
                    let last_seen_str: String = row.get(4).unwrap_or_default();
                    let (last_seen_label, last_seen_color) = format_active_status(is_online, &last_seen_str);

                    let mut last_msg_snippet = String::new();
                    let mut last_msg_ago = String::new();
                    let mut is_unread = false;

                    let true_conv_id = {
                        let mut uids = vec![my_uid.clone(), uid.clone()];
                        uids.sort();
                        format!("{}_{}", uids[0], uids[1])
                    };

                    if let Ok(mut msg_rows) = conn.query("SELECT payload, created_at, from_id, status FROM messages_v2 WHERE conversation_id = ?1 ORDER BY rowid DESC LIMIT 1", libsql::params![true_conv_id]).await {
                        if let Ok(Some(m_row)) = msg_rows.next().await {
                            let compressed: Vec<u8> = m_row.get(0).unwrap_or_default();
                            let decompressed = zstd::decode_all(compressed.as_slice()).unwrap_or(compressed);
                            let raw_text = String::from_utf8(decompressed).unwrap_or_default();
                            last_msg_snippet = if raw_text.chars().count() > 22 { format!("{}...", raw_text.chars().take(22).collect::<String>()) } else { raw_text };
                            
                            let ts: i64 = m_row.get(1).unwrap_or(0);
                            let from_id: String = m_row.get(2).unwrap_or_default();
                            let status: String = m_row.get(3).unwrap_or_default();

                            let now = chrono::Utc::now().timestamp();
                            let secs = (now - ts).max(0);
                            last_msg_ago = if secs < 60 { "now".to_string() }
                            else if secs < 3600 { format!("{}m", secs / 60) }
                            else if secs < 86400 { format!("{}h", secs / 3600) }
                            else { format!("{}d", secs / 86400) };

                            if from_id != my_uid && status != "read" {
                                is_unread = true;
                            }
                        }
                    }

                    let (subtitle_text, subtitle_color) = if !last_msg_snippet.is_empty() {
                        (format!("{} · {}", last_msg_snippet, last_msg_ago), if is_unread { "#1C1E21".to_string() } else { "#65676B".to_string() })
                    } else {
                        (last_seen_label.clone(), last_seen_color.clone())
                    };

                    let initial = fn_name.chars().next().unwrap_or('?').to_string().to_uppercase();
                    
                    let mut stack_children = vec![
                        UIComponent::CircleText { text: initial, action_id: None, color: "#FFFFFF".to_string(), bg_color: "#1877F2".to_string(), border_color: None, size: Some(56), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None }
                    ];
                    if is_online {
                        stack_children.push(UIComponent::CircleText { text: "".to_string(), action_id: None, color: "#FFFFFF".to_string(), bg_color: "#31A24C".to_string(), border_color: Some("#FFFFFF".to_string()), size: Some(16), gravity: Some("bottom_end".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None });
                    }

                    let right_indicator = if is_unread {
                        UIComponent::CircleText { text: "".to_string(), action_id: None, color: "#FFFFFF".to_string(), bg_color: "#1877F2".to_string(), border_color: None, size: Some(12), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None }
                    } else {
                        UIComponent::Text { text: "✓".to_string(), action_id: None, color: Some("#BEC3C9".to_string()), size: Some(16), align: None, bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                    };

                    children.push(
                        UIComponent::Column {
                            action_id: None,
                            align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                            children: vec![
                                UIComponent::Row {
                                    action_id: Some(format!("chat_{}", uid)), bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(8), padding_bottom: Some(8), padding_start: Some(16), padding_end: Some(16), weight: None,
                                    children: vec![
                                        UIComponent::Stack {
                                            gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(12), padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                                            children: stack_children
                                        },
                                        UIComponent::Column {
                                            action_id: None,
                                            align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0),
                                            children: vec![
                                                UIComponent::Text { text: {
                                                    let full = format!("{} {}", fn_name, ln_name).trim().to_string();
                                                    if !full.is_empty() && full != "User" {
                                                        full
                                                    } else {
                                                        let short = if uid.len() > 8 { &uid[..8] } else { &uid };
                                                        format!("User {}", short)
                                                    }
                                                }, action_id: None, color: Some("#1C1E21".to_string()), size: Some(16), align: None, bold: if is_unread { Some(true) } else { None }, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None },
                                                UIComponent::Text { text: subtitle_text, action_id: None, color: Some(subtitle_color), size: Some(13), align: None, bold: if is_unread { Some(true) } else { None }, margin_top: Some(2), margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                                            ]
                                        },
                                        right_indicator
                                    ]
                                },
                                UIComponent::Divider { margin_top: None, margin_bottom: None }
                            ]
                        }
                    );
                }
                
                if count == 0 {
                    children.push(UIComponent::Text { 
                        text: "User not found".to_string(), 
                        action_id: None, 
                        color: Some("#90949C".to_string()), 
                        size: Some(14), 
                        align: Some("center".to_string()), 
                        bold: None, 
                        margin_top: Some(32), 
                        margin_bottom: None, 
                        margin_start: None, 
                        margin_end: None, 
                        padding_top: None, 
                        padding_bottom: None, 
                        padding_start: None, 
                        padding_end: None, 
                        weight: None 
                    });
                }
            }

            let mut total_unread_chats: i64 = 0;
            if let Ok(mut unread_rows) = conn.query(
                "SELECT COUNT(DISTINCT from_id) FROM messages_v2 WHERE conversation_id LIKE '%' || ?1 || '%' AND from_id != ?1 AND status != 'read'",
                libsql::params![my_uid.clone()]
            ).await {
                if let Ok(Some(u_row)) = unread_rows.next().await {
                    total_unread_chats = u_row.get(0).unwrap_or(0);
                }
            }

            // 5. Bottom Navigation Bar (FB Messenger Style)
            let mut chats_tab_stack = vec![
                UIComponent::CircleText { text: "💬".to_string(), action_id: None, color: "#FFFFFF".to_string(), bg_color: "#000000".to_string(), border_color: None, size: Some(34), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None }
            ];
            if total_unread_chats > 0 {
                chats_tab_stack.push(
                    UIComponent::CircleText { text: total_unread_chats.to_string(), action_id: None, color: "#FFFFFF".to_string(), bg_color: "#FA383E".to_string(), border_color: Some("#FFFFFF".to_string()), size: Some(16), gravity: Some("top_end".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None }
                );
            }

            children.push(UIComponent::Row {
                action_id: None, bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(8), padding_bottom: Some(8), padding_start: Some(24), padding_end: Some(24), weight: None,
                children: vec![
                    // Tab 1: Chats
                    UIComponent::Column {
                        action_id: None,
                        align: Some("center".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0),
                        children: vec![
                            UIComponent::Stack {
                                gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                                children: chats_tab_stack
                            }
                        ]
                    },
                    // Tab 2: People
                    UIComponent::Column {
                        action_id: None,
                        align: Some("center".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0),
                        children: vec![
                            UIComponent::Stack {
                                gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                                children: vec![
                                    UIComponent::CircleText { text: "👥".to_string(), action_id: None, color: "#65676B".to_string(), bg_color: "#F0F2F5".to_string(), border_color: None, size: Some(34), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None },
                                    UIComponent::CircleText { text: "33".to_string(), action_id: None, color: "#FFFFFF".to_string(), bg_color: "#31A24C".to_string(), border_color: Some("#FFFFFF".to_string()), size: Some(16), gravity: Some("top_end".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None }
                                ]
                            }
                        ]
                    },
                    // Tab 3: Discover
                    UIComponent::Column {
                        action_id: None,
                        align: Some("center".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0),
                        children: vec![
                            UIComponent::CircleText { text: "🧭".to_string(), action_id: None, color: "#65676B".to_string(), bg_color: "#F0F2F5".to_string(), border_color: None, size: Some(34), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None }
                        ]
                    }
                ]
            });

            Ok(Json(ScreenLayout {
                screen_id: "home_screen".to_string(),
                background_color: "#FFFFFF".to_string(),
                root: UIComponent::Column {
                    action_id: None,
                    align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                    children
                }
            }))
        }

        // ── CHAT SCREEN ────────────────────────────────────────────────
        "chat" => {
            if parts.len() < 2 { return Err((StatusCode::BAD_REQUEST, "Missing uid".to_string())); }
            let other_uid = parts[1];
            
            let mut my_uid = params.get("my_uid").map(|s| s.as_str()).unwrap_or("").to_string();
            if my_uid.is_empty() {
                if let Ok(token) = extract_token(&headers) {
                    if let Ok(uid) = verify_token(&state, &token).await {
                        my_uid = uid;
                    }
                }
            }
            let mut uids = vec![my_uid.clone(), other_uid.to_string()];
            uids.sort();
            let true_conv_id = format!("{}_{}", uids[0], uids[1]);

            let conn = state.db.db.connect().unwrap();

            // Automatically mark all incoming messages in this conversation as READ
            if !my_uid.is_empty() {
                let _ = conn.execute(
                    "UPDATE messages_v2 SET status = 'read' WHERE conversation_id = ?1 AND from_id != ?2 AND status != 'read'",
                    libsql::params![true_conv_id.clone(), my_uid.clone()]
                ).await;

                // Real-time WebSocket Read Receipt push to sender
                let clients = state.clients.read().await;
                if let Some(app_clients) = clients.get("zero_lite") {
                    if let Some((_, tx)) = app_clients.get(other_uid) {
                        let push = crate::models::WsMessagePayload {
                            action: "read_receipt".to_string(),
                            message: None,
                            message_id: None,
                            conversation_id: Some(true_conv_id.clone()),
                            typing_status: None,
                        };
                        let _ = tx.send(push).await;
                    }
                }
            }

            let mut other_name = "User".to_string();
            let mut is_online = false;
            let mut last_seen_str = String::new();
            
            if let Ok(mut rows) = conn.query("SELECT first_name, last_name, is_online, last_seen FROM users WHERE uid = ?1", libsql::params![other_uid]).await {
                if let Ok(Some(row)) = rows.next().await {
                    let fn_name: String = row.get(0).unwrap_or_default();
                    let ln_name: String = row.get(1).unwrap_or_default();
                    let full = format!("{} {}", fn_name, ln_name).trim().to_string();
                    if !full.is_empty() {
                        other_name = full;
                    } else {
                        other_name = other_uid.to_string();
                    }
                    let db_online = row.get::<i64>(2).unwrap_or(0) == 1;
                    is_online = is_connected_ws(other_uid, db_online);
                    last_seen_str = row.get(3).unwrap_or_default();
                }
            }

            let initial = other_name.chars().next().unwrap_or('?').to_string().to_uppercase();
            let (status_text, status_color) = format_active_status(is_online, &last_seen_str);

            // Generate chat UI
            let mut children = vec![
                UIComponent::Row {
                    action_id: None, bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(10), padding_bottom: Some(10), padding_start: Some(10), padding_end: Some(16), weight: None,
                    children: vec![
                        UIComponent::Button { text: "←".to_string(), action_id: "go_home".to_string(), bg_color: "#FFFFFF".to_string(), text_color: Some("#1877F2".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(8), weight: None },
                        UIComponent::CircleText { text: initial.clone(), action_id: None, color: "#FFFFFF".to_string(), bg_color: "#1877F2".to_string(), border_color: None, size: Some(40), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(12) },
                        UIComponent::Column {
                            action_id: None,
                            align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0),
                            children: vec![
                                UIComponent::Text { text: other_name.clone(), action_id: None, color: Some("#1C1E21".to_string()), size: Some(16), align: None, bold: Some(true), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None },
                                UIComponent::Text { text: status_text, action_id: None, color: Some(status_color), size: Some(11), align: None, bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                            ]
                        }
                    ]
                },
                UIComponent::Divider { margin_top: None, margin_bottom: None }
            ];

            let mut msg_list = vec![];
            msg_list.push(UIComponent::Spacer { height: 12 });

            struct RawMsg {
                from_id: String,
                payload: Vec<u8>,
                status: String,
                ts: i64,
            }

            let conv_pattern = format!("%{}%", other_uid);
            let mut raw_msgs: Vec<RawMsg> = Vec::new();
            if let Ok(mut rows) = conn.query("SELECT from_id, payload, status, created_at FROM messages_v2 WHERE conversation_id = ?1 ORDER BY created_at ASC", libsql::params![true_conv_id.clone()]).await {
                while let Ok(Some(row)) = rows.next().await {
                    let from_id: String = row.get(0).unwrap_or_default();
                    let payload: Vec<u8> = row.get(1).unwrap_or_default();
                    let status: String = row.get(2).unwrap_or_else(|_| "sent".to_string());
                    let ts: i64 = row.get(3).unwrap_or(0);
                    raw_msgs.push(RawMsg { from_id, payload, status, ts });
                }
            }

            if raw_msgs.is_empty() {
                if let Ok(mut rows) = conn.query("SELECT from_id, payload, status, created_at FROM messages_v2 WHERE conversation_id LIKE ?1 ORDER BY created_at ASC", libsql::params![conv_pattern]).await {
                    while let Ok(Some(row)) = rows.next().await {
                        let from_id: String = row.get(0).unwrap_or_default();
                        let payload: Vec<u8> = row.get(1).unwrap_or_default();
                        let status: String = row.get(2).unwrap_or_else(|_| "sent".to_string());
                        let ts: i64 = row.get(3).unwrap_or(0);
                        raw_msgs.push(RawMsg { from_id, payload, status, ts });
                    }
                }
            }

            let last_seen_mine_idx = raw_msgs.iter().rposition(|m| m.from_id == my_uid && m.status == "read");
            let last_mine_idx = raw_msgs.iter().rposition(|m| m.from_id == my_uid);

            for (idx, msg) in raw_msgs.iter().enumerate() {
                let decompressed = zstd::decode_all(msg.payload.as_slice()).unwrap_or_else(|_| msg.payload.clone());
                let text = String::from_utf8(decompressed).unwrap_or_default();
                let is_mine = msg.from_id == my_uid;

                let time_str = if msg.ts > 0 {
                    chrono::NaiveDateTime::from_timestamp_opt(msg.ts, 0)
                        .map(|t| t.format("%I:%M %p").to_string().trim_start_matches('0').to_string())
                        .unwrap_or_default()
                } else {
                    String::new()
                };

                let status_str = if is_mine {
                    if Some(idx) == last_seen_mine_idx {
                        Some("seen".to_string())
                    } else if Some(idx) == last_mine_idx && idx > last_seen_mine_idx.unwrap_or(usize::MAX) {
                        if msg.status == "delivered" {
                            Some("Delivered ✓✓".to_string())
                        } else {
                            Some("Sent ✓".to_string())
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                msg_list.push(UIComponent::ChatBubble { 
                    text, 
                    is_mine, 
                    sender_initial: Some(initial.clone()),
                    recipient_initial: Some(initial.clone()),
                    status_text: status_str, 
                    time_text: Some(time_str),
                    margin_top: None, 
                    margin_bottom: Some(4) 
                });
            }
            msg_list.push(UIComponent::Spacer { height: 12 });
            
            children.push(UIComponent::Column {
                action_id: None,
                align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0),
                children: msg_list
            });

            children.push(UIComponent::Row {
                action_id: None, bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(8), padding_bottom: Some(8), padding_start: Some(8), padding_end: Some(8), weight: None,
                children: vec![
                    UIComponent::CircleText { text: "📷".to_string(), action_id: Some("action_camera".to_string()), color: "#1877F2".to_string(), bg_color: "#F0F2F5".to_string(), border_color: None, size: Some(34), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(6) },
                    UIComponent::CircleText { text: "🖼".to_string(), action_id: Some("action_gallery".to_string()), color: "#1877F2".to_string(), bg_color: "#F0F2F5".to_string(), border_color: None, size: Some(34), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(6) },
                    UIComponent::CircleText { text: "🎤".to_string(), action_id: Some("action_mic".to_string()), color: "#1877F2".to_string(), bg_color: "#F0F2F5".to_string(), border_color: None, size: Some(34), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(6) },
                    UIComponent::Input { hint: "Aa".to_string(), field_key: "message".to_string(), input_type: "text".to_string(), action_id: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(6), weight: Some(1.0) },
                    UIComponent::Button { text: "Send".to_string(), action_id: "send_message".to_string(), bg_color: "#1877F2".to_string(), text_color: Some("#FFFFFF".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None }
                ]
            });

            Ok(Json(ScreenLayout {
                screen_id: "chat_screen".to_string(),
                background_color: "#F0F2F5".to_string(),
                root: UIComponent::Column {
                    action_id: None,
                    align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                    children
                }
            }))
        }

        _ => Err((StatusCode::NOT_FOUND, "Screen not found".to_string()))
    }
}
