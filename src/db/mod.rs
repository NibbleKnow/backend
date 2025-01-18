use sqlx::{Pool, Postgres};
use crate::models::{Article, User};

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = Pool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn create_article(&self, article: &Article) -> Result<Article, sqlx::Error> {
        sqlx::query_as!(
            Article,
            r#"
            INSERT INTO articles (title, summary, content, author_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            article.title,
            article.summary,
            article.content,
            article.author_id
        )
        .fetch_one(&self.pool)
        .await
    }
}
