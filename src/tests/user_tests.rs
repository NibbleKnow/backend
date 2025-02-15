use axum::http::StatusCode;
use axum::response::Json;
use axum::Router;
use axum::routing::{get, post};
use axum::extract::State;
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::{AppState, models::User, utils::current_timestamp, enums::AppError};

#[tokio::test]
async fn test_list_users() {
    let pool = PgPool::connect("postgres://user:password@localhost/test_db").await.unwrap();
    let state = AppState {
        config: Arc::new(Config::load().unwrap()),
        db: pool,
    };

    let app = Router::new()
        .route("/users", get(list_users))
        .with_state(state.clone());

    let response = app.oneshot(Request::builder().uri("/users").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let users: Vec<User> = serde_json::from_slice(&body).unwrap();
    assert!(users.len() > 0);
}

#[tokio::test]
async fn test_create_user() {
    let pool = PgPool::connect("postgres://user:password@localhost/test_db").await.unwrap();
    let state = AppState {
        config: Arc::new(Config::load().unwrap()),
        db: pool,
    };

    let app = Router::new()
        .route("/users", post(create_user))
        .with_state(state.clone());

    let new_user = json!({
        "username": "testuser",
        "email": "testuser@example.com",
        "password_hash": "hashedpassword",
    });

    let response = app.oneshot(Request::builder().uri("/users").body(Body::from(new_user.to_string())).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let user: User = serde_json::from_slice(&body).unwrap();
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "testuser@example.com");
}

#[tokio::test]
async fn test_get_user_by_id() {
    let pool = PgPool::connect("postgres://user:password@localhost/test_db").await.unwrap();
    let state = AppState {
        config: Arc::new(Config::load().unwrap()),
        db: pool,
    };

    let app = Router::new()
        .route("/users/:id", get(get_user_by_id))
        .with_state(state.clone());

    let user_id = Uuid::new_v4();
    let response = app.oneshot(Request::builder().uri(&format!("/users/{}", user_id)).body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let user: User = serde_json::from_slice(&body).unwrap();
    assert_eq!(user.id, user_id);
}
