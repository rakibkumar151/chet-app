use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, Query, State},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::state::AppState;
use crate::models::{WsMessagePayload, Message};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct WsQueryParams {
    pub app_id: String,
    pub uid: String, // Changed to uid based on our new model
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let app_id = params.app_id;
    let uid = params.uid;

    ws.on_upgrade(move |socket| handle_socket(socket, state, app_id, uid))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, app_id: String, uid: String) {
    let (mut sender, mut receiver) = socket.split();
    
    // Using WsMessagePayload so we can send "typing", "ack", "presence" and "send" actions
    let (tx, mut rx) = mpsc::channel::<WsMessagePayload>(100);
    let conn_id = uuid::Uuid::new_v4().to_string();

    // 1. Register Client & Handle Presence (Online)
    {
        let mut clients = state.clients.write().await;
        let app_clients = clients.entry(app_id.clone()).or_default();
        app_clients.insert(uid.clone(), (conn_id.clone(), tx.clone()));
    }
    
    // Broadcast Presence
    info!("User {} came ONLINE in app {}", uid, app_id);
    let presence_payload = WsMessagePayload {
        action: "presence".to_string(),
        message: None,
        message_id: None,
        conversation_id: None,
        typing_status: Some(true), // True meaning Online
    };
    broadcast_to_friends(&state, &app_id, &uid, presence_payload).await;

    // 2. Send messages to the connected user's phone (The Downlink)
    let mut send_task = tokio::spawn(async move {
        while let Some(payload) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&payload) {
                if sender.send(WsMessage::Text(json.into())).await.is_err() {
                    break; // Phone disconnected or network dropped
                }
            }
        }
    });

    // 3. Receive messages/actions from the phone (The Uplink)
    let state_clone = Arc::clone(&state);
    let app_id_clone = app_id.clone();
    let uid_clone = uid.clone();
    
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let WsMessage::Text(text) = msg {
                if let Ok(payload) = serde_json::from_str::<WsMessagePayload>(&text) {
                    match payload.action.as_str() {
                        "send" => {
                            if let Some(mut message) = payload.message {
                                // Guaranteed Delivery Step 1: Status "sent"
                                message.status = "sent".to_string();
                                
                                // 1. Securely Save message to DB asynchronously
                                let m_clone = message.clone();
                                let db_clone = Arc::clone(&state_clone);
                                let task_app_id = app_id_clone.clone();
                                tokio::spawn(async move {
                                    if let Ok(conn) = db_clone.db.db.connect() {
                                        let sql = "INSERT INTO messages_v2 (id, app_id, conversation_id, from_id, msg_type, payload, status, likes_count, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'sent', 0, ?7)";
                                        let compressed = zstd::encode_all(m_clone.payload.as_bytes(), 3).unwrap_or_default();
                                        let _ = conn.execute(sql, libsql::params![m_clone.id, task_app_id, m_clone.conversation_id, m_clone.from_id, m_clone.msg_type, compressed, m_clone.created_at]).await;
                                    }
                                });
                                
                                info!("Routing message {} in conversation {}", message.id, message.conversation_id);
                                
                                // 2. Route to other members in the conversation
                                route_message(&state_clone, &app_id_clone, &uid_clone, message).await;
                            }
                        },
                        "ack" => {
                            // Guaranteed Delivery Step 2: Receiver phone got it
                            let m_id = payload.message_id.unwrap_or_default();
                            info!("Message {} delivered to {}", m_id, uid_clone);
                            
                            let db_clone = Arc::clone(&state_clone);
                            tokio::spawn(async move {
                                if let Ok(conn) = db_clone.db.db.connect() {
                                    let _ = conn.execute("UPDATE messages_v2 SET status = 'delivered' WHERE id = ?1", libsql::params![m_id]).await;
                                }
                            });
                        },
                        "typing" => {
                            // Typing Indicator Routing
                            info!("User {} is typing...", uid_clone);
                            let typing_payload = WsMessagePayload {
                                action: "typing".to_string(),
                                message: None,
                                message_id: None,
                                conversation_id: payload.conversation_id,
                                typing_status: payload.typing_status,
                            };
                            // Route to conversation members
                            route_action(&state_clone, &app_id_clone, typing_payload).await;
                        },
                        "like" => {
                            info!("User {} liked a message", uid_clone);
                        }
                        "button_click" | "send_test" => {
                            // App button was clicked — echo back a chat bubble via WebSocket
                            info!("User {} triggered action: {}", uid_clone, payload.action);
                            let echo = WsMessagePayload {
                                action: "ui_update".to_string(),
                                message: None,
                                message_id: None,
                                conversation_id: None,
                                typing_status: None,
                            };
                            let _ = tx.send(echo).await;
                        }
                        _ => {
                            error!("Unknown WebSocket action received");
                        }
                    }
                }
            }
        }
    });

    // 4. Wait for either task to finish (meaning network disconnect)
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // 5. Cleanup & Broadcast Presence (Offline)
    {
        let mut clients = state.clients.write().await;
        if let Some(app_clients) = clients.get_mut(&app_id) {
            if let Some((existing_id, _)) = app_clients.get(&uid) {
                if existing_id == &conn_id {
                    app_clients.remove(&uid);
                    if app_clients.is_empty() {
                        clients.remove(&app_id);
                    }
                }
            }
        }
    }
    
    info!("User {} went OFFLINE", uid);
    let offline_payload = WsMessagePayload {
        action: "presence".to_string(),
        message: None,
        message_id: None,
        conversation_id: None,
        typing_status: Some(false), // False meaning Offline
    };
    broadcast_to_friends(&state, &app_id, &uid, offline_payload).await;
}

// --- Helper Routing Functions ---

async fn broadcast_to_friends(state: &Arc<AppState>, app_id: &String, uid: &String, payload: WsMessagePayload) {
    // In a real app, query the DB for this user's friends/groups.
    // For now, this is a placeholder where we would loop over friend UIDs
    // and send the payload to their active mpsc channels if they are online.
}

async fn route_message(state: &Arc<AppState>, app_id: &String, sender_uid: &String, message: Message) {
    // 1. Query DB to get all UIDs in `message.conversation_id`
    // 2. Loop over UIDs (except sender)
    // 3. If UID is in state.clients, send a "new_message" action
    
    let clients = state.clients.read().await;
    if let Some(app_clients) = clients.get(app_id) {
        // Placeholder for DB call: let members = db.get_conversation_members(message.conversation_id);
        // For demonstration, let's say we are routing to a specific member:
        let target_uid = "some_other_uid".to_string(); 
        
        if let Some((_, tx)) = app_clients.get(&target_uid) {
            let push_payload = WsMessagePayload {
                action: "new_message".to_string(),
                message: Some(message.clone()),
                message_id: None,
                conversation_id: Some(message.conversation_id.clone()),
                typing_status: None,
            };
            let _ = tx.send(push_payload).await;
        }
    }
}

async fn route_action(state: &Arc<AppState>, app_id: &String, payload: WsMessagePayload) {
    // Similar to route_message, but for typing indicators or likes
}
