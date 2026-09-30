use libsql::Builder;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let auth_token = env::var("TURSO_AUTH_TOKEN").unwrap_or_default();
    let db = Builder::new_remote(db_url, auth_token).build().await.unwrap();
    let conn = db.connect().unwrap();

    let _ = conn.execute("DROP TABLE IF EXISTS messages_v2", ()).await;
    let _ = conn.execute("DROP TABLE IF EXISTS conversation_members", ()).await;
    let _ = conn.execute("DROP TABLE IF EXISTS conversations", ()).await;
    let _ = conn.execute("DROP TABLE IF EXISTS users", ()).await;
    println!("Dropped all tables.");
}
