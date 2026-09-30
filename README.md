# Zero Messaging API

A production-grade, super-fast B2B messaging API built with **Rust (Axum)** and **Turso (LibSQL)**. Designed to support True End-to-End Encryption (E2EE) and real-time WebSocket delivery.

## 🚀 Key Features
- **Ultra-Low Latency:** Written in Rust for maximum performance.
- **Any Data Type:** Send Text, Images (Base64), Voice (Base64), or PDFs seamlessly.
- **Smart Compression:** Integrated Facebook's `Zstandard (ZSTD)` algorithm. Compresses payloads to save up to 70% database storage.
- **E2EE Ready:** The server never touches your encryption keys. Client-side Diffie-Hellman implementations are fully supported.
- **Real-Time WebSockets:** Millisecond delivery speeds using native WebSockets.

---

## 📚 API Documentation

### 🔑 Authentication
All API calls require an API key in the headers.
- **Header Name:** `Authorization`
- **Format:** `Bearer {YOUR_API_KEY}`
- **Example:** `Bearer MySecretKey123`

---

### 1️⃣ Send Message
Sends a message (text, image, voice) to a specific user. The payload is automatically compressed using ZSTD before saving to the database.

- **Endpoint:** `POST /api/v1/messages/send`
- **Headers:** 
  - `Content-Type: application/json`
  - `Authorization: Bearer <API_KEY>`

**Request Body (JSON):**
```json
{
  "from_id": "user_a",
  "to_id": "user_b",
  "msg_type": "text", // Can be "text", "image", "voice", etc.
  "payload": "Hello, how are you?" // Plain text OR Encrypted Base64 string
}
```

**Response (200 OK):**
```json
{
  "id": "msg_9f8d7...",
  "app_id": "test_app_id",
  "from_id": "user_a",
  "to_id": "user_b",
  "msg_type": "text",
  "payload": "Hello, how are you?",
  "status": "sent", // Becomes "delivered" instantly if receiver is online
  "created_at": "2026-09-30T10:00:00Z"
}
```

---

### 2️⃣ Load Chat History
Fetches the last 50 messages between your App and a specific user. Data is fetched from Turso and decompressed in real-time.

- **Endpoint:** `GET /api/v1/messages/{channel_id}`
- **Query Parameters:**
  - `app_id`: Your Application ID
  - `user_id`: The ID of the user requesting history
- **Example:** `GET /api/v1/messages/user_b?app_id=test_app_id&user_id=user_a`
- **Headers:** 
  - `Authorization: Bearer <API_KEY>`

**Response (200 OK):** Returns an array of message objects.

---

### 3️⃣ Real-Time WebSocket Connection
Connect to receive messages in real-time. If you are connected when a message is sent to you, it will be pushed to your socket instantly, and the database status will update to `"delivered"`.

- **Endpoint:** `ws://<DOMAIN>/ws?app_id={app_id}&user_id={my_user_id}`
- **Example:** `ws://localhost:3000/ws?app_id=test_app_id&user_id=user_b`

**Incoming Message Format (Received from server):**
```json
{
  "id": "msg_9f8d7...",
  "app_id": "test_app_id",
  "from_id": "user_a",
  "to_id": "user_b",
  "msg_type": "image",
  "payload": "data:image/png;base64,...",
  "status": "sent",
  "created_at": "2026-09-30T10:05:00Z"
}
```

---

## ☁️ Deployment (Render.com)
This API is fully optimized for **Render.com**. 
1. Connect this GitHub repository to Render.
2. Select **Docker** as the environment (it will use the provided `Dockerfile`).
3. Set your Environment Variables (`TURSO_DATABASE_URL` and `TURSO_AUTH_TOKEN`).
4. Deploy! Render will build the Rust binary and bind to port `3000`.

---
*Built with ❤️ in Rust.*
