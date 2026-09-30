use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::state::AppState;
use crate::models::{Message, WsMessagePayload};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct WsQueryParams {
    pub app_id: String,
    pub user_id: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    axum::extract::Query(params): axum::extract::Query<WsQueryParams>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let app_id = params.app_id;
    let user_id = params.user_id;

    ws.on_upgrade(move |socket| handle_socket(socket, state, app_id, user_id))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, app_id: String, user_id: String) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::channel::<Message>(100);
    let conn_id = uuid::Uuid::new_v4().to_string();

    // 1. Register Client in State
    {
        let mut clients = state.clients.write().await;
        let app_clients = clients.entry(app_id.clone()).or_default();
        app_clients.insert(user_id.clone(), (conn_id.clone(), tx));
    }
    info!("User {} connected to app {} (conn_id: {})", user_id, app_id, conn_id);

    // 2. Spawn a task to send messages FROM the channel TO the websocket
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let payload = WsMessagePayload {
                action: "new_message".to_string(),
                message: Some(msg),
                message_id: None,
                to_id: None,
                text: None,
            };
            if let Ok(json) = serde_json::to_string(&payload) {
                if sender.send(WsMessage::Text(json.into())).await.is_err() {
                    break; // Client disconnected
                }
            }
        }
    });

    // 3. Receive messages FROM the websocket
    let state_clone = Arc::clone(&state);
    let app_id_clone = app_id.clone();
    let user_id_clone = user_id.clone();
    
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let WsMessage::Text(text) = msg {
                if let Ok(payload) = serde_json::from_str::<WsMessagePayload>(&text) {
                    if payload.action == "send" {
                        // TODO: Save to DB via state_clone.db
                        // Broadcast to recipient
                        info!("Received message to send to {:?}", payload.to_id);
                    } else if payload.action == "ack" {
                        info!("Received ACK for message {:?}", payload.message_id);
                        // Mark as delivered in DB
                    }
                }
            }
        }
    });

    // 4. Wait for either task to finish (disconnect)
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // 5. Cleanup on disconnect
    {
        let mut clients = state.clients.write().await;
        if let Some(app_clients) = clients.get_mut(&app_id) {
            if let Some((existing_id, _)) = app_clients.get(&user_id) {
                if existing_id == &conn_id {
                    app_clients.remove(&user_id);
                    if app_clients.is_empty() {
                        clients.remove(&app_id);
                    }
                }
            }
        }
    }
    info!("User {} disconnected from app {} (conn_id: {})", user_id, app_id, conn_id);
}
