# Zero Messaging API - Architecture & Implementation Plan

## 1. Database & Security Layer (Step 1)
**Files**: `src/db.rs`, `src/schema.sql`, `src/auth.rs`
- **Turso (libSQL)** will be used for lightning-fast edge database.
- **Tables**:
  - `apps`: Stores Developer's `app_id` and `api_key` (Hashed).
  - `messages`: Stores `id`, `app_id`, `from_id`, `to_id`, `text`, `status` (sent/delivered), `created_at`.
- **Zero Data Loss Guarantee**: All DB inserts will use ACID transactions. If DB fails, the API immediately throws an error; no false positive "Sent" statuses.
- **Auth**: Lightning-fast API Key validation middleware that caches the key in memory (Redis/Hashmap) to avoid hitting the DB for every single message.

## 2. Real-time Engine (WebSockets) (Step 2)
**Files**: `src/websocket.rs`, `src/state.rs`
- **Native WebSockets**: Built on `tokio` and `axum::extract::ws` to handle 1,000,000+ connections smoothly.
- **Connection Map**: Memory structure mapping `app_id` -> `user_id` -> `Sender<Message>`.
- **Logic**: When Developer's user connects, they authenticate with their token. Any message routed to them gets pushed instantly into their socket stream.
- **Guaranteed Delivery Check**: If the socket disconnects exactly during a message send, the system detects the dropped TCP packet and marks the message as "undelivered", saving it for when they reconnect.

## 3. The REST API (Send & Load) (Step 3)
**Files**: `src/routes.rs`, `src/handlers.rs`
- **`POST /api/v1/messages`**: For sending messages via HTTP (ideal for Chatbots). Inserts to DB -> Broadcasts to WebSocket channel if receiver is online -> Returns 200 OK.
- **`GET /api/v1/messages/:channel`**: For loading history safely. Only fetches messages matching the API Key's `app_id`. Paginates correctly to avoid server overload.

## 4. Webhooks & Background Worker (Step 4)
**Files**: `src/worker.rs`, `src/webhook.rs`
- **Webhook Dispatcher**: If a message is sent to an offline user, or if a Developer wants to feed all messages to their AI Chatbot, this worker will make a super-fast HTTP POST to the Developer's registered Webhook URL.
- **Retry Mechanism**: If the Developer's webhook server is down, Rust will retry 3 times automatically. This ensures 0 missed messages for the API consumer.

## Architecture Benefits (100% Pure & Safe):
1. Multi-tenant Isolation: A developer can never see another developer's messages because every query hardcodes `WHERE app_id = ?`.
2. Memory Safety: Rust prevents buffer overflow attacks.
3. Speed: Tokio multi-threading means messages are processed in micro-seconds.
