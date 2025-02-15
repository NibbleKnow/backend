mod api;
mod config;
mod db;
mod error;
mod models;
mod services;
mod enums;

use axum::Router;
use config::Config;
use crate::enums::AppError;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    config: Arc<Config>,
    db: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = Config::load()?;

    // Combine database initialization directly in main
    let db = PgPoolOptions::new()
        .connect(&config.database_url)
        .await
        .map_err(AppError::from)?;

    let app = Router::new()
        .nest("/api", api::routes())
        .with_state(AppState {
            config: Arc::new(config),
            db,
        });

    // Simplified server startup
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    Ok(axum::serve(listener, app).await.unwrap())
}
