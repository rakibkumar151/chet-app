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
// ─────────────────────────────────────────────
// POST /api/v1/messages/send
// ─────────────────────────────────────────────
pub async fn send_message(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<Message>, (StatusCode, String)> {
    let from_id = if let Ok(token) = extract_token(&headers) {
        if let Ok(uid) = verify_token(&state, &token).await {
            uid
        } else {
            payload.from_id.clone()
        }
    } else {
        payload.from_id.clone()
    };

    if from_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Missing sender uid".to_string()));
    }

    let (conversation_id, recipient_uid) = if payload.conversation_id.contains('_') {
        let parts: Vec<&str> = payload.conversation_id.split('_').collect();
        let other = parts.into_iter().find(|&u| u != from_id).unwrap_or("").to_string();
        (payload.conversation_id.clone(), other)
    } else {
        let rec = payload.conversation_id.clone();
        (make_conversation_id(&from_id, &rec), rec)
    };
    let msg_id = Uuid::new_v4().to_string();
    let created_at_ts = chrono::Utc::now().timestamp();

    let clients = state.clients.read().await;
    let is_delivered = if let Some(app_clients) = clients.get("zero_lite") {
        if !recipient_uid.is_empty() {
            app_clients.contains_key(&recipient_uid)
        } else {
            false
        }
    } else {
        false
    };
    let msg_status = if is_delivered { "delivered".to_string() } else { "sent".to_string() };

    let msg = Message {
        id: msg_id.clone(),
        conversation_id: conversation_id.clone(),
        from_id: from_id.clone(),
        msg_type: payload.msg_type.clone(),
        payload: payload.payload.clone(),
        status: msg_status.clone(),
        likes_count: 0,
        created_at: created_at_ts,
    };

    // 1. INSTANT REAL-TIME WEBSOCKET PUSH (0ms latency!)
    if let Some(app_clients) = clients.get("zero_lite") {
        if let Some((_, tx)) = app_clients.get(&recipient_uid) {
            let push = WsMessagePayload {
                action: "new_message".to_string(),
                message: Some(msg.clone()),
                message_id: None,
                conversation_id: Some(conversation_id.clone()),
                typing_status: None,
            };
            let _ = tx.send(push).await;
        }
        if let Some((_, tx)) = app_clients.get(&from_id) {
            let push = WsMessagePayload {
                action: "new_message".to_string(),
                message: Some(msg.clone()),
                message_id: None,
                conversation_id: Some(conversation_id.clone()),
                typing_status: None,
            };
            let _ = tx.send(push).await;
        }
    }
    drop(clients);

    // 2. NON-BLOCKING ASYNC BACKGROUND DB PERSISTENCE
    let state_db = Arc::clone(&state);
    let conv_id_clone = conversation_id.clone();
    let from_id_clone = from_id.clone();
    let payload_str = payload.payload.clone();
    let msg_type_str = payload.msg_type.clone();
    let status_to_save = msg_status.clone();

    tokio::spawn(async move {
        let payload_bytes = payload_str.as_bytes();
        let compressed_payload = zstd::encode_all(payload_bytes, 3).unwrap_or_else(|_| payload_bytes.to_vec());
        if let Ok(conn) = state_db.db.db.connect() {
            let _ = conn.execute("INSERT OR IGNORE INTO apps (app_id, api_key_hash) VALUES ('zero_lite', 'dummy')", ()).await;
            let _ = conn.execute("INSERT OR IGNORE INTO users (uid, email, first_name, last_name, gender, password, token) VALUES (?1, ?1, 'User', '', 'Unknown', '', ?1)", libsql::params![from_id_clone.clone()]).await;
            let _ = conn.execute("INSERT OR IGNORE INTO conversations (id, is_group, name) VALUES (?1, false, '')", libsql::params![conv_id_clone.clone()]).await;
            let sql = "INSERT INTO messages_v2 (id, app_id, conversation_id, from_id, msg_type, payload, status, likes_count, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, ?8)";
            let _ = conn.execute(sql, libsql::params![
                msg_id, "zero_lite".to_string(), conv_id_clone,
                from_id_clone, msg_type_str, compressed_payload, status_to_save, created_at_ts
            ]).await;
        }
    });

    // Instant HTTP 200 response
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
pub fn extract_token(headers: &HeaderMap) -> Result<String, (StatusCode, String)> {
    headers.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|t| t.to_string())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing auth token".to_string()))
}

pub async fn verify_token(state: &Arc<AppState>, token: &str) -> Result<String, (StatusCode, String)> {
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

pub async fn get_all_messages_debug(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, String)> {
    let conn = state.db.db.connect().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut rows = conn.query("SELECT id, conversation_id, from_id, payload, created_at FROM messages_v2 ORDER BY created_at DESC LIMIT 50", ())
        .await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let mut list = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        let id: String = row.get(0).unwrap_or_default();
        let conv_id: String = row.get(1).unwrap_or_default();
        let from_id: String = row.get(2).unwrap_or_default();
        let compressed: Vec<u8> = row.get(3).unwrap_or_default();
        let decompressed = zstd::decode_all(compressed.as_slice()).unwrap_or(compressed);
        let text = String::from_utf8(decompressed).unwrap_or_default();
        let created_at: i64 = row.get(4).unwrap_or(0);
        list.push(serde_json::json!({
            "id": id,
            "conversation_id": conv_id,
            "from_id": from_id,
            "text": text,
            "created_at": created_at
        }));
    }
    Ok(Json(list))
}

pub async fn get_all_users_debug(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, String)> {
    let conn = state.db.db.connect().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut rows = conn.query("SELECT uid, email, first_name, last_name, is_online, last_seen FROM users", ())
        .await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let mut list = Vec::new();
    while let Ok(Some(row)) = rows.next().await {
        let uid: String = row.get(0).unwrap_or_default();
        let email: String = row.get(1).unwrap_or_default();
        let first_name: String = row.get(2).unwrap_or_default();
        let last_name: String = row.get(3).unwrap_or_default();
        let is_online: bool = row.get::<i64>(4).unwrap_or(0) == 1;
        let last_seen: String = row.get(5).unwrap_or_default();
        list.push(serde_json::json!({
            "uid": uid,
            "email": email,
            "first_name": first_name,
            "last_name": last_name,
            "is_online": is_online,
            "last_seen": last_seen
        }));
    }
    Ok(Json(list))
}
