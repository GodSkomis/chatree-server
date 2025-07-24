use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use axum::{extract::State, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app_state::AppState, settings::{JWT_SECRET, TICKET_LIFETIME}};


#[derive(Debug, Deserialize)]
struct SignupForm {
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JWToken {
    user_id: Uuid,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct TicketResponse {
    ticket: String,
}


pub async fn sign_up(
    Json(payload): Json<SignupForm>,
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    let user_id = generate_unique_uuid(state).await;
    let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize + TICKET_LIFETIME;

    let claims = JWToken {
        user_id: user_id,
        exp: exp,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
        .expect("cannot encode jwt");

    Json(TicketResponse { ticket: token })
}

pub async fn generate_unique_uuid(state: Arc<AppState>) -> Uuid {
    let users = state.users.read().await;
    loop {
        let user_id = Uuid::new_v4();
        if !users.contains_key(&user_id) {
            return  user_id
        };
    }
}