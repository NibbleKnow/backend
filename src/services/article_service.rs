use reqwest::Error;
use redis::AsyncCommands;
use crate::{AppState, models::Article, enums::AppError};

pub async fn fetch_article_content(title: &str, source: &str) -> Result<String, Error> {
    match source {
        "wikipedia" => fetch_wikipedia_article(title).await,
        "fandom" => fetch_fandom_article(title).await,
        _ => Err(Error::new(reqwest::StatusCode::BAD_REQUEST, "Invalid source")),
    }
}

async fn fetch_wikipedia_article(title: &str) -> Result<String, Error> {
    let url = format!("https://en.wikipedia.org/api/rest_v1/page/summary/{}", title);
    let response = reqwest::get(&url).await?;
    let json: serde_json::Value = response.json().await?;
    Ok(json["extract"].as_str().unwrap_or("").to_string())
}

async fn fetch_fandom_article(title: &str) -> Result<String, Error> {
    let url = format!("https://{}.fandom.com/api.php?action=query&prop=extracts&exintro&titles={}&format=json", title);
    let response = reqwest::get(&url).await?;
    let json: serde_json::Value = response.json().await?;
    let pages = json["query"]["pages"].as_object().unwrap();
    let extract = pages.values().next().unwrap()["extract"].as_str().unwrap_or("").to_string();
    Ok(extract)
}

// Function to cache article content in Redis
pub async fn cache_article_content(state: &AppState, article: &Article) -> Result<(), AppError> {
    let mut conn = state.redis.get_async_connection().await?;
    let key = format!("article:{}", article.id);
    let value = serde_json::to_string(article)?;
    conn.set_ex(key, value, 3600).await?;
    Ok(())
}
