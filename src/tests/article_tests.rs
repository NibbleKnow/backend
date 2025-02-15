use super::*;
use tokio::runtime::Runtime;
use crate::services::article_service::{fetch_article_content, cache_article_content};
use crate::models::Article;
use crate::AppState;
use uuid::Uuid;
use time::OffsetDateTime;
use std::sync::Arc;
use crate::config::Config;
use redis::Client;

#[test]
fn test_fetch_article_content_wikipedia() {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(fetch_article_content("Rust_(programming_language)", "wikipedia"));
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Rust is a multi-paradigm"));
}

#[test]
fn test_fetch_article_content_fandom() {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(fetch_article_content("Harry_Potter", "fandom"));
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Harry Potter is a series of seven fantasy novels"));
}

#[test]
fn test_cache_article_content() {
    let rt = Runtime::new().unwrap();
    let config = Config::load().unwrap();
    let redis_client = Client::open(config.redis_url.clone()).unwrap();
    let state = AppState {
        config: Arc::new(config),
        redis: redis_client,
    };

    let article = Article {
        id: Uuid::new_v4(),
        title: "Test Article".to_string(),
        summary: "This is a test article".to_string(),
        content: "Test content".to_string(),
        author_id: Uuid::new_v4(),
        created_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
    };

    let result = rt.block_on(cache_article_content(&state, &article));
    assert!(result.is_ok());
}
