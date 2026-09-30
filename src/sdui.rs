use axum::{Json, extract::Path, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum UIComponent {
    Text { text: String, #[serde(skip_serializing_if = "Option::is_none")] color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] size: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] align: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] bold: Option<bool>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    CircleText { text: String, color: String, bg_color: String, #[serde(skip_serializing_if = "Option::is_none")] border_color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] size: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8> },
    Input { hint: String, field_key: String, input_type: String, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    Button { text: String, action_id: String, bg_color: String, #[serde(skip_serializing_if = "Option::is_none")] text_color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    ButtonOutline { text: String, action_id: String, border_color: String, #[serde(skip_serializing_if = "Option::is_none")] text_color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    Divider { #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8> },
    Spacer { height: u8 },
    ChatBubble { text: String, is_mine: bool, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8> },
    Column { children: Vec<UIComponent>, #[serde(skip_serializing_if = "Option::is_none")] align: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
    Row { children: Vec<UIComponent>, #[serde(skip_serializing_if = "Option::is_none")] bg_color: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] margin_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] margin_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_top: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_bottom: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_start: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] padding_end: Option<u8>, #[serde(skip_serializing_if = "Option::is_none")] weight: Option<f32> },
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
                    UIComponent::Text { text: "English (US)  ⌄".to_string(), color: Some("#606770".to_string()), size: Some(13), align: Some("end".to_string()), bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None },
                    UIComponent::CircleText { text: "f".to_string(), color: "#1877F2".to_string(), bg_color: "#FFFFFF".to_string(), border_color: Some("#DADDE1".to_string()), size: Some(72), margin_top: Some(16), margin_bottom: Some(8), margin_start: None, margin_end: None },
                    UIComponent::Spacer { height: 24 },
                    UIComponent::Input { hint: "Mobile number or email".to_string(), field_key: "email".to_string(), input_type: "email".to_string(), margin_top: None, margin_bottom: Some(12), margin_start: None, margin_end: None, weight: None },
                    UIComponent::Input { hint: "Password".to_string(), field_key: "password".to_string(), input_type: "password".to_string(), margin_top: None, margin_bottom: Some(16), margin_start: None, margin_end: None, weight: None },
                    UIComponent::Button { text: "Log in".to_string(), action_id: "do_login".to_string(), bg_color: "#1877F2".to_string(), text_color: Some("#FFFFFF".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None },
                    UIComponent::Text { text: "Forgot password?".to_string(), color: Some("#1877F2".to_string()), size: Some(14), align: Some("center".to_string()), bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(16), padding_bottom: Some(24), padding_start: None, padding_end: None, weight: None },
                    UIComponent::Divider { margin_top: None, margin_bottom: None },
                    UIComponent::Spacer { height: 20 },
                    UIComponent::ButtonOutline { text: "Create new account".to_string(), action_id: "go_signup".to_string(), border_color: "#1877F2".to_string(), text_color: Some("#1877F2".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None },
                    UIComponent::Spacer { height: 32 },
                    UIComponent::Text { text: "Meta ∞".to_string(), color: Some("#606770".to_string()), size: Some(13), align: Some("center".to_string()), bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                ]
            }
        })),

        // ── SIGNUP SCREEN ──────────────────────────────────────────────
        "signup" => Ok(Json(ScreenLayout {
            screen_id: "signup_screen".to_string(),
            background_color: "#FFFFFF".to_string(),
            root: UIComponent::Column {
                align: Some("center".to_string()),
                padding_top: Some(32), padding_bottom: Some(32), padding_start: Some(24), padding_end: Some(24),
                margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None,
                children: vec![
                    UIComponent::Text { text: "Create a new account".to_string(), color: Some("#1C1E21".to_string()), size: Some(22), align: Some("center".to_string()), bold: Some(true), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None },
                    UIComponent::Text { text: "It's quick and easy.".to_string(), color: Some("#606770".to_string()), size: Some(14), align: Some("center".to_string()), bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(4), padding_bottom: Some(20), padding_start: None, padding_end: None, weight: None },
                    UIComponent::Divider { margin_top: None, margin_bottom: None },
                    UIComponent::Spacer { height: 16 },
                    UIComponent::Row {
                        bg_color: None, margin_top: None, margin_bottom: Some(12), margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None,
                        children: vec![
                            UIComponent::Input { hint: "First name".to_string(), field_key: "first_name".to_string(), input_type: "text".to_string(), margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(8), weight: Some(1.0) },
                            UIComponent::Input { hint: "Last name".to_string(), field_key: "last_name".to_string(), input_type: "text".to_string(), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: Some(1.0) }
                        ]
                    },
                    UIComponent::Input { hint: "Mobile number or email".to_string(), field_key: "email".to_string(), input_type: "email".to_string(), margin_top: None, margin_bottom: Some(12), margin_start: None, margin_end: None, weight: None },
                    UIComponent::Input { hint: "Gender (Male/Female)".to_string(), field_key: "gender".to_string(), input_type: "text".to_string(), margin_top: None, margin_bottom: Some(20), margin_start: None, margin_end: None, weight: None },
                    UIComponent::Button { text: "Sign Up".to_string(), action_id: "do_signup".to_string(), bg_color: "#1877F2".to_string(), text_color: Some("#FFFFFF".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, weight: None },
                    UIComponent::Spacer { height: 12 },
                    UIComponent::Text { text: "Already have an account?".to_string(), color: Some("#1877F2".to_string()), size: Some(14), align: Some("center".to_string()), bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
                ]
            }
        })),

        // ── USERS LIST ─────────────────────────────────────────────────
        "users" => {
            let mut children = vec![
                UIComponent::Row {
                    bg_color: Some("#FFFFFF".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(14), padding_bottom: Some(14), padding_start: Some(16), padding_end: Some(16), weight: None,
                    children: vec![
                        UIComponent::Text { text: "Zero Chat".to_string(), color: Some("#1877F2".to_string()), size: Some(20), align: None, bold: Some(true), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0) },
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
                            bg_color: Some("#FFFFFF".to_string()), margin_top: Some(1), margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(12), padding_bottom: Some(12), padding_start: Some(16), padding_end: Some(16), weight: None,
                            children: vec![
                                UIComponent::CircleText { text: initial, color: "#FFFFFF".to_string(), bg_color: "#1877F2".to_string(), border_color: None, size: Some(48), margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(12) },
                                UIComponent::Column {
                                    align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0),
                                    children: vec![
                                        UIComponent::Text { text: format!("{} {}", fn_name, ln_name), color: Some("#1C1E21".to_string()), size: Some(15), align: None, bold: Some(true), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None },
                                        UIComponent::Text { text: status.to_string(), color: Some(status_color.to_string()), size: Some(12), align: None, bold: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: None }
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

        // ── CHAT SCREEN ────────────────────────────────────────────────
        "chat" => {
            if parts.len() < 2 { return Err((StatusCode::BAD_REQUEST, "Missing uid".to_string())); }
            let uid = parts[1];
            
            // Generate chat UI
            let mut children = vec![
                UIComponent::Row {
                    bg_color: Some("#FFFFFF".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(12), padding_bottom: Some(12), padding_start: Some(16), padding_end: Some(16), weight: None,
                    children: vec![
                        UIComponent::Button { text: "←".to_string(), action_id: "refresh_users".to_string(), bg_color: "#FFFFFF".to_string(), text_color: Some("#1877F2".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(12), weight: None },
                        UIComponent::Text { text: "Chat".to_string(), color: Some("#1C1E21".to_string()), size: Some(16), align: None, bold: Some(true), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0) }
                    ]
                }
            ];

            let mut msg_list = vec![];
            
            // Fetch messages for conversation
            // Here we assume we want all messages for this uid but we need to know MY uid.
            // Since this is just returning the screen layout, we don't know who is requesting unless we pass it.
            // In a real app we'd pass token in get_ui_screen. For simplicity now, let's just show an input.
            
            msg_list.push(UIComponent::Spacer { height: 16 });
            
            children.push(UIComponent::Column {
                align: None, margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: None, padding_bottom: None, padding_start: None, padding_end: None, weight: Some(1.0),
                children: msg_list
            });

            children.push(UIComponent::Row {
                bg_color: Some("#FFFFFF".to_string()), margin_top: None, margin_bottom: None, margin_start: None, margin_end: None, padding_top: Some(8), padding_bottom: Some(8), padding_start: Some(12), padding_end: Some(12), weight: None,
                children: vec![
                    UIComponent::Input { hint: "Aa".to_string(), field_key: "message".to_string(), input_type: "text".to_string(), margin_top: None, margin_bottom: None, margin_start: None, margin_end: Some(8), weight: Some(1.0) },
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
