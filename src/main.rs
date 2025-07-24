use axum::{
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::{RwLock, broadcast};

use crate::{app_state::AppState, cache::cache::TimedCache};

mod websocket;
mod app_state;
mod cache;
mod auth;
pub mod settings;


#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    user: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct WsQuery {
    token: String,
}

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[tokio::main]
async fn main() {
    // let subscriber = FmtSubscriber::new();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        // .with_file(true)
        // .with_line_number(true)
        // .with_thread_ids(true)
        // .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let (tx, _rx) = broadcast::channel(100);

    let state = AppState {
        clients: Arc::new(RwLock::new(HashMap::new())),
        tickets: Arc::new(RwLock::new(TimedCache::new())),
        users: Arc::new(RwLock::new(HashMap::new())),
        tx: tx
    };
    let app = Router::new()
        // .route("/ws", get(ws_handler))
        .route("/ws", get(websocket::websocket_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// async fn ws_handler(
//     ws: WebSocketUpgrade,
//     ConnectInfo(addr): ConnectInfo<SocketAddr>,
//     State(clients): State<Clients>,
//     Query(query): Query<WsQuery>,
// ) -> impl IntoResponse {
//     match validate_token(&query.token) {
//         Ok(username) => {
//             tracing::info!("🔐 {} подключился как {}", addr, username);
//             ws.on_upgrade(move |socket| handle_socket(socket, username, clients))
//         }
//         Err(err) => {
//             tracing::warn!("⛔ Неверный токен от {}: {}", addr, err);
//             axum::http::StatusCode::UNAUTHORIZED.into_response()
//         }
//     }
// }

// fn validate_token(token: &str) -> Result<String, String> {
//     let decoded = decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(JWT_SECRET),
//         &Validation::new(Algorithm::HS256),
//     ).map_err(|e| e.to_string())?;

//     Ok(decoded.claims.sub)
// }

// async fn handle_socket(mut socket: WebSocket, username: String, clients: Clients) {
//     let uuid = Uuid::new_v4();
//     let (tx, mut rx) = broadcast::channel::<ChatMessage>(100);
//     clients.write().await.insert(uuid, tx.clone());

//     // отправка в сокет
//     let recv_task = tokio::spawn(async move {
//         while let Ok(msg) = rx.recv().await {
//             if msg.user != username {
//                 if let Ok(json) = serde_json::to_string(&msg) {
//                     let _ = socket.send(Message::Text(Utf8Bytes::from(json))).await;
//                 }
//             }
//         }
//     });

//     // получение от клиента
//     while let Some(Ok(msg)) = socket.recv().await {
//         if let axum::extract::ws::Message::Text(text) = msg {
//             let trimmed = text.trim();
//             if trimmed.is_empty() {
//                 continue;
//             }

//             let full_msg = ChatMessage {
//                 user: username.clone(),
//                 message: trimmed.to_string(),
//             };

//             let clients_map = clients.read().await;
//             for (other_id, tx) in clients_map.iter() {
//                 if *other_id != uuid {
//                     let _ = tx.send(full_msg.clone());
//                 }
//             }
//         }
//     }

//     clients.write().await.remove(&uuid);
//     tracing::info!("❌ {} отключился", username);
// }

