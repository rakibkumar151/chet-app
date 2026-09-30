use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppClient {
    pub app_id: String,
    pub api_key_hash: String,
    pub webhook_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub app_id: String,
    pub from_id: String,
    pub to_id: String,
    pub msg_type: String, // "text", "image", "voice", etc.
    pub payload: String,  // The actual text or base64 data
    pub status: String, // "sent", "delivered", "read"
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsMessagePayload {
    pub action: String, // "send", "ack", "load_history"
    pub message: Option<Message>,
    pub message_id: Option<String>, // for acks
    pub to_id: Option<String>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMessageRequest {
    pub from_id: String,
    pub to_id: String,
    pub msg_type: String,
    pub payload: String,
}
