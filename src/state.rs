use std::collections::HashMap;
use tokio::sync::{mpsc, RwLock};

use crate::db::DbContext;
use crate::models::WsMessagePayload;

pub struct AppState {
    pub db: DbContext,
    // app_id -> (user_id -> (connection_id, sender channel))
    pub clients: RwLock<HashMap<String, HashMap<String, (String, mpsc::Sender<WsMessagePayload>)>>>,
}

impl AppState {
    pub fn new(db: DbContext) -> Self {
        Self {
            db,
            clients: RwLock::new(HashMap::new()),
        }
    }
}
