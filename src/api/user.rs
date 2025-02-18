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

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(list_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user_by_id))
}

async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&state.db)
        .await
        .map_err(AppError::from)?;
    Ok(Json(users))
}


async fn create_user(State(state): State<AppState>, Json(payload): Json<User>) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, username, email, password_hash, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        Uuid::new_v4(),
        payload.username,
        payload.email,
        payload.password_hash,
        current_timestamp()
    )
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)?;
    Ok(Json(user))
}

async fn get_user_by_id(State(state): State<AppState>, axum::extract::Path(id): axum::extract::Path<Uuid>) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&state.db)
        .await
        .map_err(AppError::from)?;
    Ok(Json(user))
}
