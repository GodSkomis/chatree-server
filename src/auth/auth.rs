use std::sync::Arc;
use axum::{extract::{Query, State}, http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Deserialize;

use crate::{
    app_state::AppState, auth::{jwt_authorization::{JWTAuthorize, JWTResponse, JWToken},
    password::{generate_password_hash, verify_password}, ticket::TicketQuery}, core::ErrorResponse, models::{errors::ModelError,user::{NewUser, User}}
};


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

#[axum::debug_handler]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(form): Json<LoginForm>,
) -> Result<Json<JWTResponse>, Response> {

    let dto = User::authorize(form.username.clone(), &state.pool)
        .await
        .map_err(|err| ModelError::into_error_response(err, None, None))?;

    let dto = dto.ok_or_else(|| {
        let error = Json(ErrorResponse {
            error: format!("User with given username ({}) not found", form.username),
        });
        (StatusCode::BAD_REQUEST, error).into_response()
    })?;

    if !verify_password(form.password, dto.hashed_password) {
        let error = Json(ErrorResponse {
            error: "Wrong password".to_string()
        });
        return Err((StatusCode::UNAUTHORIZED, error).into_response())
    }

    let jwt = JWToken::new(dto.id);
    let token = jwt.encode();
    tracing::debug!("JWT ({}): `{}`", dto.id, token);
    Ok(Json(JWTResponse { token: token }))
    
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
        status: None,
        bio: None
    };
    let new_user_id = match new_user.insert(&state.pool).await {
        Ok(_user_id) => _user_id,
        Err(err) => return Err(ModelError::into_error_response(err, None, None))
    };
    let token = JWToken::new(new_user_id);
    tracing::debug!("NewUser: {}", new_user_id);
    Ok(Json(JWTResponse { token: token.encode() }))
}


pub async fn ticket(
    JWTAuthorize(jwt): JWTAuthorize,
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    (StatusCode::CREATED, state.tickets.generate(jwt.claims.user_id).await)
}


pub async fn revoke_ticket (
    JWTAuthorize(jwt): JWTAuthorize,
    Query(ticket_query): Query<TicketQuery>,
    State(state): State<Arc<AppState>>
) -> Response {
    match state.tickets.validated_remove(jwt.claims.user_id, &ticket_query.ticket).await {
        Ok(_) => (StatusCode::OK, Json(ErrorResponse { error: format!("Ticket `{}` has been successfully revoked", &ticket_query.ticket) })).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(ErrorResponse{ error: err.to_string() })).into_response()
    }
}