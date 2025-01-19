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
use crate::enums::AppError;

// Constants for mock data
const MOCK_EMAIL: &str = "mockemail@email.com";
const MOCK_USERNAME: &str = "mockuser";

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(list_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user_by_id))
}

// Extracted function: encapsulates user creation logic
fn create_mock_user() -> User {
    User {
        id: Uuid::new_v4(),
        email: MOCK_EMAIL.to_string(),
        password_hash: Uuid::new_v4().as_hyphenated().to_string(),
        username: MOCK_USERNAME.to_string(),
        created_at: current_timestamp(),
    }
}

async fn list_users(State(_state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users: Vec<User> = vec![];
    Ok(Json(users)) // Ensure proper Result wrapping
}

async fn create_user(State(_state): State<AppState>, Json(_payload): Json<CreateUser>) -> Result<Json<User>, AppError> {
    let user = create_mock_user();
    Ok(Json(user))
}

async fn get_user_by_id(State(_state): State<AppState>) -> Result<Json<User>, AppError> {
    todo!()
}