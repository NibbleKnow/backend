mod api;
mod config;
mod db;
mod error;
mod models;
mod services;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use config::Config;
use error::AppError;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    config: Arc<Config>,
    db: PgPool,
    redis: redis::Client,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = Config::load()?;
    let db = PgPool::connect(&config.database_url).await?;
    let redis = redis::Client::open(config.redis_url.as_str())?;

    let state = AppState {
        config: Arc::new(config),
        db,
        redis,
    };

    let app = Router::new()
        .nest("/api", api::routes())
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
