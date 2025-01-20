use axum::{extract::State, Json, http::StatusCode, Router};
use axum::routing::{get, post};
use uuid::Uuid;
use time::OffsetDateTime;
use crate::{AppState, models::Article, enums::AppError};

// Predefined route constants for easier modification and reusability
const ROUTE_ARTICLES: &str = "/articles";
const ROUTE_ARTICLE_BY_ID: &str = "/articles/:id";

pub fn article_routes() -> Router<AppState> {
    Router::new()
        .route(ROUTE_ARTICLES, get(list_articles))
        .route(ROUTE_ARTICLE_BY_ID, get(get_article))
        .route(ROUTE_ARTICLES, post(create_article))
}

async fn list_articles(State(state): State<AppState>) -> Result<Json<Vec<Article>>, AppError> {
    let articles = fetch_articles(&state, None).await?;
    Ok(Json(articles)) // Ensures consistent Json<Result> response
}

async fn create_article(
    State(state): State<AppState>,
    Json(input): Json<CreateArticle>,
) -> Result<StatusCode, AppError> {
    let new_article = create_new_article(input);
    let article = sqlx::query_as!(
        Article,
        "INSERT INTO articles (title, summary, content, author_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        new_article.title,
        new_article.summary,
        new_article.content,
        new_article.author_id,
        new_article.created_at,
        new_article.updated_at
    )
    .fetch_one(&state.db)
    .await
    .map_err(AppError::from)?;
    Ok(StatusCode::CREATED)
}

async fn get_article(State(state): State<AppState>, axum::extract::Path(id): axum::extract::Path<Uuid>) -> Result<Json<Article>, AppError> {
    let article = sqlx::query_as!(Article, "SELECT * FROM articles WHERE id = $1", id)
        .fetch_one(&state.db)
        .await
        .map_err(AppError::from)?;
    Ok(Json(article))
}

// Helper function to create a new article
fn create_new_article(input: CreateArticle) -> Article {
    Article {
        id: Uuid::new_v4(),
        title: input.title,
        summary: input.summary,
        content: input.content,
        author_id: input.author_id,
        created_at: current_timestamp(),
        updated_at: current_timestamp(),
    }
}

async fn fetch_articles(state: &AppState, _filter: Option<String>) -> Result<Vec<Article>, AppError> {
    let articles = sqlx::query_as!(Article, "SELECT * FROM articles")
        .fetch_all(&state.db)
        .await
        .map_err(AppError::from)?;
    Ok(articles)
}

fn current_timestamp() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}
