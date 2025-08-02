use axum::response::IntoResponse;
use axum::routing::post;
use axum::{
    routing::get,
    Router,
};

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use tracing::Level;
use std::{
    collections::HashMap, env, sync::Arc
};
use tokio::sync::{RwLock, broadcast};
use dotenv::dotenv;

use crate::models::AppPool;
use crate::settings::MAX_CONNECTIONS;
use crate::{app_state::AppState, cache::cache::TimedCache, models::prelude::SnowflakeGenerator};

mod websocket;
mod app_state;
mod cache;
mod auth;
pub mod schema;
pub mod models;
pub mod settings;
pub mod core;


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
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG) // Уровень логирования (например, INFO)
        .init();
    // let subscriber = FmtSubscriber::new();
    // let subscriber = tracing_subscriber::fmt()
    //     .compact()
    //     // .with_file(true)
    //     // .with_line_number(true)
    //     // .with_thread_ids(true)
    //     // .with_target(false)
    //     .finish();
    // tracing::subscriber::set_global_default(subscriber).unwrap();

    let (tx, _rx) = broadcast::channel(100);
    let pool = create_pg_pool().await;

    let state = AppState {
        clients: Arc::new(RwLock::new(HashMap::new())),
        tickets: Arc::new(RwLock::new(TimedCache::new())),
        users: Arc::new(RwLock::new(HashMap::new())),
        snowflake_generator: Arc::new(SnowflakeGenerator::new()),
        tx: tx,
        pool: pool,
    };

    let app = Router::new()
        // .route("/ws", get(ws_handler))
        .route("/ws", get(websocket::websocket_handler))
        .route("/signup", post(auth::auth::sign_up))
        .route("/login", post(auth::auth::login))
        .route("/ping", get(ping))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();
    
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



// Local pool
async fn create_pg_pool() -> AppPool {
    let database_url = env::var("DATABASE_URL").unwrap();

    PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(&database_url).await.unwrap()
}


async fn ping() -> impl IntoResponse {
    "Pong!"
}


// fn create_pg_pool() -> Pool<AsyncPgConnection> {
// use diesel_async::pooled_connection::deadpool::Pool;
// use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

//     let max_size = 6;
//     let database_url = env::var("DATABASE_URL").unwrap();
//     let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
//     Pool::builder(manager)
//         .max_size(max_size)
//         .build()
//         .unwrap()
// }

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

