use axum::{Router, Json, routing::post};
use openmls::prelude::tls_codec::Serialize;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// use crate::models::{AppState, User};
use super::mls::{generate_credential, generate_key_package};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user_id: String,
    pub key_package: Vec<u8>,
}

static STATE: once_cell::sync::Lazy<Arc<Mutex<AppState>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(AppState::default())));

pub fn routes() -> Router {
    Router::new().route("/register", post(register_user))
}

async fn register_user(Json(payload): Json<RegisterRequest>) -> Json<RegisterResponse> {
    let (cred, signer) = generate_credential(&payload.name);
    let kp_bundle = generate_key_package(cred.clone(), &signer);

    let kp = kp_bundle.key_package().clone();
    let serialized = kp
        .tls_serialize_detached()
        .expect("serialization failed");

    let user = User {
        id: Uuid::new_v4(),
        name: payload.name.clone(),
        key_package: Some(kp),
    };

    STATE.lock().unwrap().users.insert(payload.name.clone(), user);

    Json(RegisterResponse {
        user_id: payload.name,
        key_package: serialized,
    })
}
