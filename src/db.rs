use libsql::Builder;
use std::env;

pub struct DbContext {
    pub db: libsql::Database,
}

impl DbContext {
    pub async fn new() -> Self {
        dotenv::dotenv().ok();
        let db_url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
        let auth_token = env::var("TURSO_AUTH_TOKEN").unwrap_or_default();

        let db = Builder::new_remote(db_url, auth_token)
            .build()
            .await
            .expect("Failed to connect to Turso database");

        Self { db }
    }

    pub async fn migrate(&self) {
        let conn = self.db.connect().unwrap();
        
        // 0. Apps table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS apps (
                app_id TEXT PRIMARY KEY,
                api_key_hash TEXT NOT NULL,
                webhook_url TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )", ()
        ).await.unwrap();

        let _ = conn.execute(
            "INSERT OR IGNORE INTO apps (app_id, api_key_hash) VALUES ('test_app_id', 'dummy_hash')", ()
        ).await;

        // 1. Users table (Strictly Server-Side, NO raw user data sent to app)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                uid TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                profile_pic TEXT,
                is_online BOOLEAN DEFAULT false,
                last_seen DATETIME DEFAULT CURRENT_TIMESTAMP
            )", ()
        ).await.unwrap();

        // 2. Conversations table (Supports groups)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                is_group BOOLEAN DEFAULT false,
                name TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )", ()
        ).await.unwrap();

        // 3. Conversation Members table (for fast routing)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS conversation_members (
                conversation_id TEXT NOT NULL,
                uid TEXT NOT NULL,
                PRIMARY KEY (conversation_id, uid),
                FOREIGN KEY (conversation_id) REFERENCES conversations(id),
                FOREIGN KEY (uid) REFERENCES users(uid)
            )", ()
        ).await.unwrap();

        // 4. Secure Messages table (Replaced to_id with conversation_id)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages_v2 (
                id TEXT PRIMARY KEY,
                app_id TEXT NOT NULL,
                conversation_id TEXT NOT NULL,
                from_id TEXT NOT NULL,
                msg_type TEXT NOT NULL,
                payload BLOB NOT NULL,
                status TEXT DEFAULT 'sent',
                likes_count INTEGER DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (app_id) REFERENCES apps(app_id),
                FOREIGN KEY (conversation_id) REFERENCES conversations(id),
                FOREIGN KEY (from_id) REFERENCES users(uid)
            )", ()
        ).await.unwrap();
            
        println!("FB-Lite secure database tables initialized successfully!");
    }
}
