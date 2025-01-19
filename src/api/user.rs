use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    Router,
    routing::{get, post},
    extract::State,
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{models::User, utils::current_timestamp, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(list_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
}

async fn list_users(
    State(state): State<AppState>
) -> Result<Json<Vec<User>>, AppError> {
    (StatusCode::OK, Json(""))
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>
) -> Result<Json<User>, AppError> {
    let user = User {
        id: Uuid::new_v4(),
        email: "mockemail@email.com".to_string(),
        password_hash: Uuid::new_v4().as_str(),
        username: "mockuser".to_string(),
        created_at: current_timestamp(),
    };

    (StatusCode::CREATED, Json(user))
}

async fn get_user(
    State(state): State<AppState>
) -> Result<Json<User>, AppError> {
    todo!()
}
