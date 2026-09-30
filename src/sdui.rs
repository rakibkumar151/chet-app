use axum::{Json, extract::Path, extract::Query, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum UIComponent {
    Text { text: String, #[serde(skip_serializing_if = "Option::is_none")] action_id: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] size: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] align: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] bold: Option<bool>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    CircleText { text: String, color: String, bg_color: String, #[serde(skip_serializing_if = "Option::is_none")] border_color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] size: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] gravity: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8> },
    Input { hint: String, field_key: String, input_type: String, #[serde(skip_serializing_if = "Option::is_none")] action_id: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    Button { text: String, action_id: String, bg_color: String, #[serde(skip_serializing_if = "Option::is_none")] text_color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    ButtonOutline { text: String, action_id: String, border_color: String, #[serde(skip_serializing_if = "Option::is_none")] text_color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    Divider { #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8> },
    Spacer { height: u8 },
    ChatBubble { text: String, is_mine: bool, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8> },
    Column { children: Vec<UIComponent>, #[serde(skip_serializing_if = "Option::is_none")] align: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    Row { children: Vec<UIComponent>, #[serde(skip_serializing_if = "Option::is_none")] bg_color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] gravity: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    HorizontalScroll { children: Vec<UIComponent>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    Stack { children: Vec<UIComponent>, #[serde(skip_serializing_if = "Option::is_none")] gravity: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenLayout {
    pub screen_id: String,
    pub background_color: String,
    pub root: UIComponent,
}

pub async fn get_ui_screen(
    State(state): State<Arc<AppState>>,
    Path(screen_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ScreenLayout>, (StatusCode, String)> {

    let parts: Vec<&str> = screen_name.split('/').collect();
    let name = parts[0];

    match name {
        // ── LOGIN SCREEN (FB LITE STYLE) ──────────────────────────────
        "login" => Ok(Json(ScreenLayout {
            screen_id: "login_screen".to_string(),
            background_color: "#FFFFFF".to_string(),
            root: UIComponent::Column {
                align: Some("center".to_string()),
                padding_top: Some(40), padding_bottom: Some(32), padding_start: Some(32), padding_end: Some(32),
                margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None,
                children: vec![
                    UIComponent::Text { text: "English (US)  ⌄".to_string(), action_id: None, color: Some("#606770".to_string()), size: Some(13), align: Some("end".to_string()), bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None },
                    UIComponent::CircleText { text: "f".to_string(), color: "#1877F2".to_string(), bg_color: "#FFFFFF".to_string(), border_color: Some("#DADDE1".to_string()), size: Some(72), gravity: None, margin_top: Some(16), margin_bottom: Some(8), margin_start: None, margin_end: None },
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
                align: Some("center".to_string()),
                padding_top: Some(16), padding_bottom: Some(32), padding_start: Some(24), padding_end: Some(24),
                margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None,
                children: vec![
                    // Back Arrow
                    UIComponent::Row {
                        bg_color: None, gravity: None, margin_top: None, margin_bottom: Some(16), margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
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
                        bg_color: None, gravity: None, margin_top: None, margin_bottom: Some(12), margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
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
                    bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(14), padding_bottom: Some(14), padding_start: Some(16), padding_end: Some(16), weight: None,
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
                            bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: Some(1), margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(12), padding_bottom: Some(12), padding_start: Some(16), padding_end: Some(16), weight: None,
                            children: vec![
                                UIComponent::CircleText { text: initial, color: "#FFFFFF".to_string(), bg_color: "#1877F2".to_string(), border_color: None, size: Some(48), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(12) },
                                UIComponent::Column {
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
                    align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                    children
                }
            }))
        }

        // ── HOME / CHATS SCREEN ───────────────────────────────────────
        "home" => {
            let mut children = vec![
                // 1. Top Bar
                UIComponent::Row {
                    bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(12), padding_bottom: Some(12), padding_start: Some(16), padding_end: Some(16), weight: None,
                    children: vec![
                        UIComponent::CircleText { text: "ME".to_string(), color: "#FFFFFF".to_string(), bg_color: "#A0A0A0".to_string(), border_color: None, size: Some(36), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(12) },
                        UIComponent::Text { text: "Chats".to_string(), action_id: None, color: Some("#1C1E21".to_string()), size: Some(24), align: None, bold: Some(true), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0) },
                        UIComponent::CircleText { text: "📷".to_string(), color: "#1C1E21".to_string(), bg_color: "#F0F2F5".to_string(), border_color: None, size: Some(36), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(12) },
                        UIComponent::CircleText { text: "✏".to_string(), color: "#1C1E21".to_string(), bg_color: "#F0F2F5".to_string(), border_color: None, size: Some(36), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None },
                    ]
                },
                // 2. Search Bar
                UIComponent::Row {
                    bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(4), padding_bottom: Some(12), padding_start: Some(16), padding_end: Some(16), weight: None,
                    children: vec![
                        UIComponent::Input { hint: "🔍 Search".to_string(), field_key: "search".to_string(), input_type: "text".to_string(), action_id: Some("search_users".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: Some(1.0) }
                    ]
                }
            ];

            // 3. Active Users (Story Tray)
            let conn = state.db.db.connect().unwrap();
            let mut active_tray = vec![];
            
            active_tray.push(
                UIComponent::Column {
                    align: Some("center".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(16), padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                    children: vec![
                        UIComponent::Stack {
                            gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                            children: vec![
                                UIComponent::CircleText { text: "ME".to_string(), color: "#FFFFFF".to_string(), bg_color: "#A0A0A0".to_string(), border_color: Some("#E4E6EB".to_string()), size: Some(56), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None },
                                UIComponent::CircleText { text: "+".to_string(), color: "#1C1E21".to_string(), bg_color: "#FFFFFF".to_string(), border_color: Some("#E4E6EB".to_string()), size: Some(18), gravity: Some("bottom_end".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None }
                            ]
                        }
                    ]
                }
            );

            if let Ok(mut rows) = conn.query("SELECT uid, first_name, is_online FROM users LIMIT 10", ()).await {
                while let Ok(Some(row)) = rows.next().await {
                    let fn_name: String = row.get(1).unwrap_or_default();
                    let is_online = row.get::<i64>(2).unwrap_or(0) == 1;
                    let initial = fn_name.chars().next().unwrap_or('?').to_string().to_uppercase();
                    
                    let ring_color = if is_online { Some("#1877F2".to_string()) } else { None };
                    
                    let mut stack_children = vec![
                        UIComponent::CircleText { text: initial, color: "#FFFFFF".to_string(), bg_color: "#A0A0A0".to_string(), border_color: ring_color, size: Some(56), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None }
                    ];
                    if is_online {
                        stack_children.push(UIComponent::CircleText { text: "".to_string(), color: "#FFFFFF".to_string(), bg_color: "#31A24C".to_string(), border_color: Some("#FFFFFF".to_string()), size: Some(16), gravity: Some("bottom_end".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None });
                    }
                    
                    active_tray.push(
                        UIComponent::Column {
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
                conn.query("SELECT uid, first_name, last_name, is_online FROM users", ()).await
            } else {
                conn.query("SELECT uid, first_name, last_name, is_online FROM users WHERE first_name LIKE ?1 OR last_name LIKE ?1", libsql::params![q_param.clone()]).await
            };
            
            if let Ok(mut rows) = query_res {
                while let Ok(Some(row)) = rows.next().await {
                    let uid: String = row.get(0).unwrap_or_default();
                    let fn_name: String = row.get(1).unwrap_or_default();
                    let ln_name: String = row.get(2).unwrap_or_default();
                    let is_online = row.get::<i64>(3).unwrap_or(0) == 1;

                    let initial = fn_name.chars().next().unwrap_or('?').to_string().to_uppercase();
                    let is_unread = uid.len() % 2 == 0; 
                    
                    let mut stack_children = vec![
                        UIComponent::CircleText { text: initial, color: "#FFFFFF".to_string(), bg_color: "#A0A0A0".to_string(), border_color: None, size: Some(56), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None }
                    ];
                    if is_online {
                        stack_children.push(UIComponent::CircleText { text: "".to_string(), color: "#FFFFFF".to_string(), bg_color: "#31A24C".to_string(), border_color: Some("#FFFFFF".to_string()), size: Some(16), gravity: Some("bottom_end".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None });
                    }

                    children.push(
                        UIComponent::Row {
                            bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(8), padding_bottom: Some(8), padding_start: Some(16), padding_end: Some(16), weight: None,
                            children: vec![
                                UIComponent::Stack {
                                    gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(12), padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                                    children: stack_children
                                },
                                UIComponent::Column {
                                    align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0),
                                    children: vec![
                                        UIComponent::Text { text: format!("{} {}", fn_name, ln_name), action_id: Some(format!("chat_{}", uid)), color: Some("#1C1E21".to_string()), size: Some(16), align: None, bold: Some(is_unread), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None },
                                        UIComponent::Text { text: "Love it! • 9m".to_string(), action_id: None, color: Some(if is_unread { "#1C1E21".to_string() } else { "#606770".to_string() }), size: Some(14), align: None, bold: Some(is_unread), margin_top: Some(4), margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                                    ]
                                },
                                if is_unread {
                                    UIComponent::CircleText { text: "".to_string(), color: "#1877F2".to_string(), bg_color: "#1877F2".to_string(), border_color: None, size: Some(12), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None }
                                } else {
                                    UIComponent::Text { text: "✓".to_string(), action_id: None, color: Some("#BEC3C9".to_string()), size: Some(12), align: None, bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                                }
                            ]
                        }
                    );
                }
            }

            Ok(Json(ScreenLayout {
                screen_id: "home_screen".to_string(),
                background_color: "#FFFFFF".to_string(),
                root: UIComponent::Column {
                    align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                    children
                }
            }))
        }

        // ── CHAT SCREEN ────────────────────────────────────────────────
        "chat" => {
            if parts.len() < 2 { return Err((StatusCode::BAD_REQUEST, "Missing uid".to_string())); }
            let other_uid = parts[1];
            
            let my_uid = params.get("my_uid").map(|s| s.as_str()).unwrap_or("");
            let mut uids = vec![my_uid.to_string(), other_uid.to_string()];
            uids.sort();
            let true_conv_id = format!("{}_{}", uids[0], uids[1]);

            let conn = state.db.db.connect().unwrap();
            let mut other_name = "User".to_string();
            if let Ok(mut rows) = conn.query("SELECT first_name, last_name FROM users WHERE uid = ?1", libsql::params![other_uid]).await {
                if let Ok(Some(row)) = rows.next().await {
                    let fn_name: String = row.get(0).unwrap_or_default();
                    let ln_name: String = row.get(1).unwrap_or_default();
                    other_name = format!("{} {}", fn_name, ln_name);
                }
            }

            // Generate chat UI
            let mut children = vec![
                UIComponent::Row {
                    bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(12), padding_bottom: Some(12), padding_start: Some(16), padding_end: Some(16), weight: None,
                    children: vec![
                        UIComponent::Button { text: "←".to_string(), action_id: "refresh_users".to_string(), bg_color: "#FFFFFF".to_string(), text_color: Some("#1877F2".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(12), weight: None },
                        UIComponent::Text { text: other_name, action_id: None, color: Some("#1C1E21".to_string()), size: Some(16), align: None, bold: Some(true), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0) }
                    ]
                }
            ];

            let mut msg_list = vec![];
            msg_list.push(UIComponent::Spacer { height: 16 });

            if let Ok(mut rows) = conn.query("SELECT from_id, payload FROM messages_v2 WHERE conversation_id = ?1 ORDER BY created_at ASC", libsql::params![true_conv_id]).await {
                while let Ok(Some(row)) = rows.next().await {
                    let from_id: String = row.get(0).unwrap_or_default();
                    let payload: Vec<u8> = row.get(1).unwrap_or_default();
                    let text = String::from_utf8_lossy(&payload).to_string();
                    let is_mine = from_id == my_uid;
                    msg_list.push(UIComponent::ChatBubble { text, is_mine, margin_top: None, margin_bottom: Some(4) });
                }
            }            msg_list.push(UIComponent::Spacer { height: 16 });
            
            children.push(UIComponent::Column {
                align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0),
                children: msg_list
            });

            children.push(UIComponent::Row {
                bg_color: Some("#FFFFFF".to_string()), gravity: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(8), padding_bottom: Some(8), padding_start: Some(12), padding_end: Some(12), weight: None,
                children: vec![
                    UIComponent::Input { hint: "Aa".to_string(), field_key: "message".to_string(), input_type: "text".to_string(), action_id: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(8), weight: Some(1.0) },
                    UIComponent::Button { text: "Send".to_string(), action_id: "send_message".to_string(), bg_color: "#1877F2".to_string(), text_color: Some("#FFFFFF".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None }
                ]
            });

            Ok(Json(ScreenLayout {
                screen_id: "chat_screen".to_string(),
                background_color: "#F0F2F5".to_string(),
                root: UIComponent::Column {
                    align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                    children
                }
            }))
        }

        _ => Err((StatusCode::NOT_FOUND, "Screen not found".to_string()))
    }
}
