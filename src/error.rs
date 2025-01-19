use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::enums::error::AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = self.to_status_and_message();
        (status_code, json!({ "error": error_message })).into_response()
    }
}

impl AppError {
    fn to_status_and_message(&self) -> (StatusCode, &str) {
        match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Redis(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cache error"),
            AppError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Server configuration error"),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            _ => (StatusCode::BAD_REQUEST, "Bad request"),
        }
    }
}