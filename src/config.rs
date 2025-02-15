use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub wikipedia_api_base_url: String,
    pub fandom_api_base_url: String,
}

impl Config {
    pub fn load() -> Result<Self, AppError> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| AppError::Config("DATABASE_URL not set".into()))?,
            redis_url: env::var("REDIS_URL")
                .map_err(|_| AppError::Config("REDIS_URL not set".into()))?,
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| AppError::Config("JWT_SECRET not set".into()))?,
            wikipedia_api_base_url: env::var("WIKIPEDIA_API_BASE_URL")
                .map_err(|_| AppError::Config("WIKIPEDIA_API_BASE_URL not set".into()))?,
            fandom_api_base_url: env::var("FANDOM_API_BASE_URL")
                .map_err(|_| AppError::Config("FANDOM_API_BASE_URL not set".into()))?,
        })
    }
}
