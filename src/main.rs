mod db;
mod handlers;
mod models;
mod state;
mod websocket;
mod worker;
mod sdui;

use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber;

fn main() {
    // INCREASE DEFAULT STACK SIZE TO PREVENT OVERFLOWS ON WINDOWS
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(8 * 1024 * 1024) // 8 MB stack size
        .build()
        .unwrap();
        
    rt.block_on(async {
        // Spawn inside tokio to ensure it runs on a worker thread with 8MB stack
        tokio::spawn(async {
            tracing_subscriber::fmt::init();
            
            println!("Starting Zero Messaging API...");
            
            // 1. Initialize Database
            let db_context = db::DbContext::new().await;
            db_context.migrate().await;
            
            let shared_state = Arc::new(state::AppState::new(db_context));

            use tower_http::cors::{Any, CorsLayer};
            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any);

            // 2. Setup Routes
            let app = Router::new()
                .route("/health", get(|| async { "OK" }))
                .route("/ws", get(websocket::ws_handler))
                .route("/api/v1/screen/{screen_name}", get(sdui::get_ui_screen))
                .route("/api/v1/auth/signup", axum::routing::post(handlers::signup))
                .route("/api/v1/auth/login", axum::routing::post(handlers::login))
                .route("/api/v1/users", get(handlers::get_users))
                .route("/api/v1/messages/send", axum::routing::post(handlers::send_message))
                .route("/api/v1/messages/{other_uid}", get(handlers::get_history))
                .with_state(shared_state)
                .layer(cors);

            // 3. Start Server
            let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
            let addr = format!("0.0.0.0:{}", port);
            
            println!("Server running on http://{}", addr);
            let listener = TcpListener::bind(&addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }).await.unwrap();
    });
}
