use axum::{extract::State, Json, http::StatusCode, Router};
use axum::routing::{get, post};
use uuid::Uuid;
use time::OffsetDateTime;
use crate::{AppState, models::Article, enums::AppError};
use reqwest::Error; // Import reqwest for HTTP requests
use crate::services::article_service::{fetch_article_content, cache_article_content}; // Import the new fetch_article_content and cache_article_content functions
use redis::AsyncCommands; // Import Redis commands

// Predefined route constants for easier modification and reusability
const ROUTE_ARTICLES: &str = "/articles";
const ROUTE_ARTICLE_BY_ID: &str = "/articles/:id";
const ROUTE_FETCH_ARTICLE: &str = "/fetch_article"; // New route for fetching article content

pub fn article_routes() -> Router<AppState> {
    Router::new()
        .route(ROUTE_ARTICLES, get(list_articles))
        .route(ROUTE_ARTICLE_BY_ID, get(get_article))
        .route(ROUTE_ARTICLES, post(create_article))
        .route(ROUTE_FETCH_ARTICLE, get(fetch_article)) // Add the new route
}

async fn list_articles(State(state): State<AppState>) -> Result<Json<Vec<Article>>, AppError> {
    let articles = fetch_articles(&state, None).await?;
    Ok(Json(articles)) // Ensures consistent Json<Result> response
}

async fn create_article(
    State(state): State<AppState>,
    Json(input): Json<CreateArticle>,
) -> Result<StatusCode, AppError> {
    let content = fetch_article_content(&input.title).await?;
    let new_article = create_new_article(input, content);
    cache_article_content(&state, &new_article).await?; // Cache the article content in Redis
    Ok(StatusCode::CREATED)
}

async fn get_article(State(_state): State<AppState>) -> Result<Json<Article>, AppError> {
    todo!("Implement fetching an article by ID")
}

// New function to fetch article content from Wikipedia and Fandom APIs
async fn fetch_article(State(state): State<AppState>, Json(input): Json<FetchArticleRequest>) -> Result<Json<FetchArticleResponse>, AppError> {
    let content = fetch_article_content(&input.title, &input.source).await?;
    let response = FetchArticleResponse { content };
    Ok(Json(response))
}

// Helper function to create a new article
fn create_new_article(input: CreateArticle, content: String) -> Article {
    Article {
        id: Uuid::new_v4(),
        title: input.title,
        summary: input.summary,
        content,
        author_id: input.author_id,
        created_at: current_timestamp(),
        updated_at: current_timestamp(),
    }
}

async fn fetch_articles(state: &AppState, _filter: Option<String>) -> Result<Vec<Article>, AppError> {
    Ok(Vec::new())
}

fn current_timestamp() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

// Function to cache article content in Redis
async fn cache_article_content(state: &AppState, article: &Article) -> Result<(), AppError> {
    let mut conn = state.redis.get_async_connection().await?;
    let key = format!("article:{}", article.id);
    let value = serde_json::to_string(article)?;
    conn.set_ex(key, value, 3600).await?;
    Ok(())
}

// Unit tests for fetch_article_content function
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;
    use crate::services::article_service::fetch_article_content;

    #[test]
    fn test_fetch_article_content() {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(fetch_article_content("Rust_(programming_language)"));
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Rust is a multi-paradigm"));
    }
}

// Structs for the new fetch_article function
#[derive(Deserialize)]
struct FetchArticleRequest {
    title: String,
    source: String,
}

#[derive(Serialize)]
struct FetchArticleResponse {
    content: String,
}
