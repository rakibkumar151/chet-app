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
        // Create apps table for Multi-tenant support
        conn
            .execute(
                "CREATE TABLE IF NOT EXISTS apps (
                    app_id TEXT PRIMARY KEY,
                    api_key_hash TEXT NOT NULL,
                    webhook_url TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )",
                (),
            )
            .await
            .unwrap();

        // Insert a default app for testing
        let _ = conn
            .execute(
                "INSERT OR IGNORE INTO apps (app_id, api_key_hash) VALUES ('test_app_id', 'dummy_hash')",
                (),
            )
            .await;

        // Create messages_v2 table (Optimized storage with BLOB)
        conn
            .execute(
                "CREATE TABLE IF NOT EXISTS messages_v2 (
                    id TEXT PRIMARY KEY,
                    app_id TEXT NOT NULL,
                    from_id TEXT NOT NULL,
                    to_id TEXT NOT NULL,
                    msg_type TEXT NOT NULL,
                    payload BLOB NOT NULL,
                    status TEXT DEFAULT 'sent',
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (app_id) REFERENCES apps(app_id)
                )",
                (),
            )
            .await
            .unwrap();
            
        println!("Database tables initialized successfully!");
    }
}
