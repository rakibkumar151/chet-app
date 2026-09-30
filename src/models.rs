use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppClient {
    pub app_id: String,
    pub api_key_hash: String,
    pub webhook_url: Option<String>,
}

// 1. User Profile & Presence Management
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub uid: String,
    pub username: String,
    pub profile_pic: Option<String>, // URL or Base64 Encrypted String
    pub is_online: bool,             // Shows if active
    pub last_seen: i64,              // Timestamp for "Last seen at..."
}

// 2. Conversation handling (Supports both 1-on-1 and Group Chats)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: String,
    pub is_group: bool,
    pub name: Option<String>, // Group Name (Null if 1-on-1)
    pub members: Vec<String>, // List of UIDs (2 people, or 100 people)
}

// 3. Message Structure with Likes & Delivery Status
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub conversation_id: String, // Links to Conversation (instead of just to_id)
    pub from_id: String,
    pub msg_type: String,        // "text", "image", "voice"
    pub payload: String,         // Encrypted payload
    pub status: String,          // "sent", "delivered", "read"
    pub likes_count: i32,        // Message Like/Reaction system
    pub created_at: i64,         // Unix Timestamp
}

// 4. WebSocket Payload (Realtime Actions)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsMessagePayload {
    pub action: String, // "send", "ack", "typing", "presence", "like"
    pub message: Option<Message>,
    pub message_id: Option<String>, 
    pub conversation_id: Option<String>,
    pub typing_status: Option<bool>, // Realtime typing indicator
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMessageRequest {
    pub from_id: String,
    pub conversation_id: String,
    pub msg_type: String,
    pub payload: String,
}
