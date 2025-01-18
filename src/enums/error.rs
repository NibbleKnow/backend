use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError{
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Not Found {0}")]
    NotFound(String),
    #[error("Bad Request {0}")]
    BadRequest(String)
}
