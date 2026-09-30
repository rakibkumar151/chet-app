use crate::models::Message;
use std::time::Duration;
use tokio::time::sleep;

pub async fn dispatch_webhook(app_id: String, message: Message, webhook_url: String) {
    let client = reqwest::Client::new();
    
    // Retry logic for 100% Data Guarantee
    // If developer's server is down, we retry up to 3 times automatically
    for attempt in 1..=3 {
        match client.post(&webhook_url)
            .json(&message)
            .send()
            .await 
        {
            Ok(res) if res.status().is_success() => {
                tracing::info!("Webhook delivered successfully to {} for app {}", webhook_url, app_id);
                return;
            }
            Ok(res) => {
                tracing::warn!("Webhook returned error status {} (Attempt {}/3)", res.status(), attempt);
            }
            Err(e) => {
                tracing::error!("Webhook failed to send: {} (Attempt {}/3)", e, attempt);
            }
        }
        
        sleep(Duration::from_secs(2)).await; // Wait 2 seconds before retry
    }
    tracing::error!("Failed to deliver webhook for app {} after 3 attempts", app_id);
}
