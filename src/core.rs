use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::models::errors::ModelError;


#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}


impl ModelError {
    pub fn into_error_response(
        err: ModelError,
        client_err_status: Option<StatusCode>,
        unexpected_err_status: Option<StatusCode>)
        -> axum::response::Response {

        match err {
            ModelError::ClientError(error) => {
                (
                    match client_err_status {
                        Some(status) => status,
                        None => StatusCode::BAD_REQUEST
                    }, 
                    Json(ErrorResponse { error })
                ).into_response()
            }
            ModelError::UnexpectedError(error) => {
                (
                    match unexpected_err_status {
                        Some(status) => status,
                        None => StatusCode::INTERNAL_SERVER_ERROR,
                    }, 
                    Json(ErrorResponse { error })
                ).into_response()
            }
        }
    }
}