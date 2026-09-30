use serde::{Deserialize, Serialize};

// Auth
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignupRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub token: String,
    pub uid: String,
    pub first_name: String,
}

// User Profile
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPublic {
    pub uid: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub is_online: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppClient {
    pub app_id: String,
    pub api_key_hash: String,
    pub webhook_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub uid: String,
    pub username: String,
    pub profile_pic: Option<String>,
    pub is_online: bool,
    pub last_seen: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: String,
    pub is_group: bool,
    pub name: Option<String>,
    pub members: Vec<String>,
}

// Message Structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub from_id: String,
    pub msg_type: String,
    pub payload: String,
    pub status: String,
    pub likes_count: i32,
    pub created_at: i64,
}

// WebSocket Payload (Realtime Actions)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsMessagePayload {
    pub action: String,
    pub message: Option<Message>,
    pub message_id: Option<String>,
    pub conversation_id: Option<String>,
    pub typing_status: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMessageRequest {
    pub from_id: String,
    pub conversation_id: String,
    pub msg_type: String,
    pub payload: String,
}
