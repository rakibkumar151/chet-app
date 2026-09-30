use axum::{
    extract::{State, Path},
    http::{HeaderMap, StatusCode},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::AppState;
use crate::models::{Message, SendMessageRequest, WsMessagePayload, SignupRequest, LoginRequest, AuthResponse, UserPublic};

// ─────────────────────────────────────────────
// POST /api/v1/auth/signup
// ─────────────────────────────────────────────
pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let conn = state.db.db.connect().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Check if email already exists
    let existing = conn.query(
        "SELECT uid FROM users WHERE email = ?1",
        libsql::params![req.email.clone()]
    ).await;

    if let Ok(mut rows) = existing {
        if let Ok(Some(_)) = rows.next().await {
            return Err((StatusCode::CONFLICT, "Email already registered".to_string()));
        }
    }

    let uid = Uuid::new_v4().to_string();
    let token = Uuid::new_v4().to_string(); // Simple token — swap for JWT in prod

    conn.execute(
        "INSERT INTO users (uid, email, first_name, last_name, gender, password, token) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        libsql::params![uid.clone(), req.email, req.first_name.clone(), req.last_name, req.gender, req.password, token.clone()]
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        uid,
        first_name: req.first_name,
    }))
}

// ─────────────────────────────────────────────
// POST /api/v1/auth/login
// ─────────────────────────────────────────────
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let conn = state.db.db.connect().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut rows = conn.query(
        "SELECT uid, first_name, token FROM users WHERE email = ?1 AND password = ?2",
        libsql::params![req.email, req.password]
    ).await.map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()))?;

    if let Ok(Some(row)) = rows.next().await {
        let uid: String = row.get(0).unwrap_or_default();
        let first_name: String = row.get(1).unwrap_or_default();
        let token: String = row.get(2).unwrap_or_default();

        Ok(Json(AuthResponse { token, uid, first_name }))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Email not found".to_string()))
    }
}

// ─────────────────────────────────────────────
// GET /api/v1/users  (requires Bearer token)
// ─────────────────────────────────────────────
pub async fn get_users(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<UserPublic>>, (StatusCode, String)> {
    let token = extract_token(&headers)?;
    let my_uid = verify_token(&state, &token).await?;

    let conn = state.db.db.connect().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut rows = conn.query(
        "SELECT uid, first_name, last_name, gender, is_online FROM users WHERE uid != ?1 ORDER BY first_name ASC",
        libsql::params![my_uid]
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut users = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        users.push(UserPublic {
            uid: row.get(0).unwrap_or_default(),
            first_name: row.get(1).unwrap_or_default(),
            last_name: row.get(2).unwrap_or_default(),
            gender: row.get(3).unwrap_or_default(),
            is_online: row.get::<i64>(4).unwrap_or(0) == 1,
        });
    }

    Ok(Json(users))
}

// ─────────────────────────────────────────────
// POST /api/v1/messages/send
// ─────────────────────────────────────────────
pub async fn send_message(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<Message>, (StatusCode, String)> {
    let token = extract_token(&headers)?;
    let from_id = verify_token(&state, &token).await?;

    let conversation_id = make_conversation_id(&from_id, &payload.conversation_id);
    let msg_id = Uuid::new_v4().to_string();
    let created_at_ts = chrono::Utc::now().timestamp();

    let payload_bytes = payload.payload.as_bytes();
    let compressed_payload = zstd::encode_all(payload_bytes, 3).unwrap_or_else(|_| payload_bytes.to_vec());

    let conn = state.db.db.connect().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let sql = "INSERT INTO messages_v2 (id, app_id, conversation_id, from_id, msg_type, payload, status, likes_count, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'sent', 0, ?7)";
    conn.execute(sql, libsql::params![
        msg_id.clone(), "zero_lite".to_string(), conversation_id.clone(),
        from_id.clone(), payload.msg_type.clone(), compressed_payload, created_at_ts
    ]).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let msg = Message {
        id: msg_id,
        conversation_id: conversation_id.clone(),
        from_id: from_id.clone(),
        msg_type: payload.msg_type,
        payload: payload.payload,
        status: "sent".to_string(),
        likes_count: 0,
        created_at: created_at_ts,
    };

    // Push real-time to recipient via WebSocket
    let recipient_uid = payload.conversation_id.clone(); // conversation_id is the other user's uid
    let clients = state.clients.read().await;
    if let Some(app_clients) = clients.get("zero_lite") {
        if let Some((_, tx)) = app_clients.get(&recipient_uid) {
            let push_payload = WsMessagePayload {
                action: "new_message".to_string(),
                message: Some(msg.clone()),
                message_id: None,
                conversation_id: Some(conversation_id),
                typing_status: None,
            };
            let _ = tx.send(push_payload).await;
        }
    }

    Ok(Json(msg))
}

// ─────────────────────────────────────────────
// GET /api/v1/messages/{other_uid}
// ─────────────────────────────────────────────
pub async fn get_history(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(other_uid): Path<String>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let token = extract_token(&headers)?;
    let my_uid = verify_token(&state, &token).await?;

    let conversation_id = make_conversation_id(&my_uid, &other_uid);

    let conn = state.db.db.connect().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let sql = "SELECT id, conversation_id, from_id, msg_type, payload, status, likes_count, created_at FROM messages_v2 WHERE conversation_id = ?1 ORDER BY created_at ASC LIMIT 50";

    let mut messages = Vec::new();
    let mut rows = conn.query(sql, libsql::params![conversation_id])
        .await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    while let Ok(Some(row)) = rows.next().await {
        let compressed: Vec<u8> = row.get(4).unwrap_or_default();
        let decompressed = zstd::decode_all(compressed.as_slice()).unwrap_or(compressed);
        let payload_str = String::from_utf8(decompressed).unwrap_or_default();

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

    Ok(Json(messages))
}

// ─────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────
fn extract_token(headers: &HeaderMap) -> Result<String, (StatusCode, String)> {
    headers.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|t| t.to_string())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing auth token".to_string()))
}

async fn verify_token(state: &Arc<AppState>, token: &str) -> Result<String, (StatusCode, String)> {
    let conn = state.db.db.connect().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut rows = conn.query("SELECT uid FROM users WHERE token = ?1", libsql::params![token.to_string()])
        .await.map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    if let Ok(Some(row)) = rows.next().await {
        Ok(row.get(0).unwrap_or_default())
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))
    }
}

/// Makes a stable, order-independent conversation ID from two UIDs
fn make_conversation_id(uid_a: &str, uid_b: &str) -> String {
    let mut parts = [uid_a, uid_b];
    parts.sort();
    format!("{}_{}", parts[0], parts[1])
}
