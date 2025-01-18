use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde_json::json;
use crate::enums::error::AppError;


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Redis(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cache error"),
            AppError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Server configuration error"),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.as_str()),
        };

        (status, json!({ "error": message })).into_response()
    }
}
