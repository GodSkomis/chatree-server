use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use rand::{self, distr::Alphanumeric, Rng};
use axum::{extract::State, http::HeaderMap, response::{ErrorResponse, IntoResponse}, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use tokio::task;

use crate::{app_state::AppState, auth::password::generate_password_hash, cache::cache::Cache, models::user::{NewUser, User}, settings::{JWT_SECRET, TICKET_LENGTH, TICKET_LIFETIME}};


#[derive(Debug, Deserialize)]
pub struct SignupForm {
    pub username: String,
    pub name: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWToken {
    user_id: i64,
    exp: usize,
}


impl JWToken {
    fn new(user_id: i64) -> Self {
        Self {
            user_id: user_id,
            exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize + TICKET_LIFETIME
        }
    }

    fn encode(&self) -> String {
        encode(&Header::default(), &self, &EncodingKey::from_secret(JWT_SECRET)).unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct JWTResponse {
    token: String
}


#[axum::debug_handler]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(form): Json<LoginForm>,
) -> impl IntoResponse {
    // let user: User = match get_user() {
    //     Some(_user) => _user,
    //     None => return Json(ErrorResponse::from("User not found"))
    // };

    // let jwt = JWToken::new(user.id);
    
    // Json(JWTResponse { token: token.encode() })
    Json(JWTResponse { token: "OK".to_string() })
}


#[axum::debug_handler]
pub async fn sign_up(
    State(state): State<Arc<AppState>>,
    Json(form): Json<SignupForm>,
) -> impl IntoResponse {
    let user_id = state.snowflake_generator.generate_id().await;
    let hashed_password = generate_password_hash(form.password);
    let new_user = NewUser {
        id: user_id,
        username: form.username,
        name: form.name,
        hashed_password: hashed_password,
    };
    let new_user_id = new_user.insert(&state.pool).await;
    let token = JWToken::new(new_user_id);
    tracing::debug!("NewUser: {}", new_user_id);
    Json(JWTResponse { token: token.encode() })
}


pub async fn ticket(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>
) -> String {
    let ticket = task::spawn_blocking(|| generate_random_ticket()).await.unwrap();
    {
        let tickets = state.tickets.write().await;
        // tickets.set(ticket.clone(), user_id, None);
    }

    ticket
}


fn generate_random_ticket() -> String {
    let ticket: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(TICKET_LENGTH)
        .map(char::from)
        .collect();
    ticket
}


pub async fn validate_ticket(
    ticket: String,
    State(state): State<Arc<AppState>>
) -> bool {

    let tickets = state.tickets.write().await;
    if let None = tickets.get(&ticket) {
        return false
    }

    tickets.remove(&ticket).unwrap();
    true
}