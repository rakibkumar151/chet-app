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
    // 1. API Key Validation (In production, verify against `apps` table)
    let auth_header = headers.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    
    if !auth_header.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Missing or invalid API Key".to_string()));
    }
    
    // For demonstration, we assume the API key maps to "test_app_id"
    let app_id = "test_app_id".to_string();
    let from_id = payload.from_id.clone();

    // 2. COMPRESS DATA (To Save Turso Size)
    let msg_id = Uuid::new_v4().to_string();
    
    // Compress payload string into ZSTD bytes
    let payload_bytes = payload.payload.as_bytes();
    let compressed_payload = zstd::encode_all(payload_bytes, 3).unwrap_or_else(|_| payload_bytes.to_vec());

    // 3. Save to Turso Database
    let conn = state.db.db.connect().unwrap();
    let sql = "INSERT INTO messages_v2 (id, app_id, from_id, to_id, msg_type, payload, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'sent')";
    let args = libsql::params![
        msg_id.clone(),
        app_id.clone(),
        from_id.clone(),
        payload.to_id.clone(),
        payload.msg_type.clone(),
        compressed_payload
    ];
    
    if let Err(e) = conn.execute(sql, args).await {
        tracing::error!("Failed to save message to DB: {}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()));
    }

    let msg = Message {
        id: msg_id.clone(),
        app_id: app_id.clone(),
        from_id: from_id.clone(),
        to_id: payload.to_id.clone(),
        msg_type: payload.msg_type,
        payload: payload.payload, // Return decompressed/original payload in JSON response
        status: "sent".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    // 4. Super-Fast Real-Time WebSocket Push (If receiver is online)
    let sender_channel = {
        let clients = state.clients.read().await;
        clients.get(&app_id)
            .and_then(|app_clients| app_clients.get(&payload.to_id))
            .map(|(_, tx)| tx.clone())
    };

    if let Some(channel) = sender_channel {
        // Receiver is online, push to socket!
        let _ = channel.send(msg.clone()).await;
        
        // Optional: Update status to 'delivered' in DB asynchronously
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
    Path(channel_id): Path<String>, // Usually the to_id
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    // 1. API Key Validation
    let auth_header = headers.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    
    if !auth_header.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Missing or invalid API Key".to_string()));
    }
    
    let app_id = "test_app_id".to_string();

    // 2. Fetch from Turso (Very Fast Query)
    let conn = state.db.db.connect().unwrap();
    let sql = "SELECT id, app_id, from_id, to_id, msg_type, payload, status, created_at FROM messages_v2 WHERE app_id = ?1 AND (to_id = ?2 OR from_id = ?2) ORDER BY created_at DESC LIMIT 50";
    
    let mut messages = Vec::new();
    
    match conn.query(sql, libsql::params![app_id, channel_id]).await {
        Ok(mut rows) => {
            while let Ok(Some(row)) = rows.next().await {
                // Decompress BLOB
                let compressed_payload: Vec<u8> = row.get(5).unwrap_or_default();
                let decompressed_bytes = zstd::decode_all(compressed_payload.as_slice()).unwrap_or(compressed_payload);
                let payload_str = String::from_utf8(decompressed_bytes).unwrap_or_default();

                messages.push(Message {
                    id: row.get(0).unwrap_or_default(),
                    app_id: row.get(1).unwrap_or_default(),
                    from_id: row.get(2).unwrap_or_default(),
                    to_id: row.get(3).unwrap_or_default(),
                    msg_type: row.get(4).unwrap_or_default(),
                    payload: payload_str,
                    status: row.get(6).unwrap_or_default(),
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
