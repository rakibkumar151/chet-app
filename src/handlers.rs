use axum::{
    extract::{State, Path},
    http::{HeaderMap, StatusCode},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::AppState;
use crate::models::{Message, SendMessageRequest, WsMessagePayload};

pub async fn send_message(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<Message>, (StatusCode, String)> {
    let auth_header = headers.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    
    if !auth_header.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Missing or invalid API Key".to_string()));
    }
    
    let app_id = "test_app_id".to_string();
    let from_id = payload.from_id.clone();
    let conversation_id = payload.conversation_id.clone();
    let msg_id = Uuid::new_v4().to_string();
    let created_at_ts = chrono::Utc::now().timestamp();
    
    let payload_bytes = payload.payload.as_bytes();
    let compressed_payload = zstd::encode_all(payload_bytes, 3).unwrap_or_else(|_| payload_bytes.to_vec());

    let conn = state.db.db.connect().unwrap();
    let sql = "INSERT INTO messages_v2 (id, app_id, conversation_id, from_id, msg_type, payload, status, likes_count, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'sent', 0, ?7)";
    let args = libsql::params![
        msg_id.clone(),
        app_id.clone(),
        conversation_id.clone(),
        from_id.clone(),
        payload.msg_type.clone(),
        compressed_payload,
        created_at_ts
    ];
    
    if let Err(e) = conn.execute(sql, args).await {
        tracing::error!("Failed to save message to DB: {}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()));
    }

    let msg = Message {
        id: msg_id.clone(),
        conversation_id: conversation_id.clone(),
        from_id: from_id.clone(),
        msg_type: payload.msg_type,
        payload: payload.payload,
        status: "sent".to_string(),
        likes_count: 0,
        created_at: created_at_ts,
    };

    let sender_channel = {
        let clients = state.clients.read().await;
        clients.get(&app_id)
            .and_then(|app_clients| app_clients.get(&conversation_id))
            .map(|(_, tx)| tx.clone())
    };

    if let Some(channel) = sender_channel {
        let push_payload = WsMessagePayload {
            action: "new_message".to_string(),
            message: Some(msg.clone()),
            message_id: None,
            conversation_id: Some(conversation_id.clone()),
            typing_status: None,
        };
        let _ = channel.send(push_payload).await;
        
        let state_clone = Arc::clone(&state);
        let m_id = msg_id.clone();
        tokio::spawn(async move {
            if let Ok(conn) = state_clone.db.db.connect() {
                let _ = conn.execute("UPDATE messages_v2 SET status = 'delivered' WHERE id = ?1", libsql::params![m_id]).await;
            }
        });
    }

    Ok(Json(msg))
}

pub async fn get_history(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(channel_id): Path<String>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let auth_header = headers.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    
    if !auth_header.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Missing or invalid API Key".to_string()));
    }
    
    let app_id = "test_app_id".to_string();

    let conn = state.db.db.connect().unwrap();
    let sql = "SELECT id, conversation_id, from_id, msg_type, payload, status, likes_count, created_at FROM messages_v2 WHERE app_id = ?1 AND conversation_id = ?2 ORDER BY created_at DESC LIMIT 50";
    
    let mut messages = Vec::new();
    
    match conn.query(sql, libsql::params![app_id, channel_id]).await {
        Ok(mut rows) => {
            while let Ok(Some(row)) = rows.next().await {
                let compressed_payload: Vec<u8> = row.get(4).unwrap_or_default();
                let decompressed_bytes = zstd::decode_all(compressed_payload.as_slice()).unwrap_or(compressed_payload);
                let payload_str = String::from_utf8(decompressed_bytes).unwrap_or_default();

                messages.push(Message {
                    id: row.get(0).unwrap_or_default(),
                    conversation_id: row.get(1).unwrap_or_default(),
                    from_id: row.get(2).unwrap_or_default(),
                    msg_type: row.get(3).unwrap_or_default(),
                    payload: payload_str,
                    status: row.get(5).unwrap_or_default(),
                    likes_count: row.get(6).unwrap_or_default(),
                    created_at: row.get(7).unwrap_or_default(),
                });
            }
        }
        Err(e) => {
            tracing::error!("Failed to fetch history from DB: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()));
        }
    }

    Ok(Json(messages))
}
