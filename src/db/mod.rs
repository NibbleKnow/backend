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
            "INSERT INTO articles (title, summary, content, author_id) VALUES ($1, $2, $3, $4) RETURNING *",
            article.title,
            article.summary,
            article.content,
            article.author_id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (id, username, email, password_hash, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.created_at
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn fetch_users(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn fetch_user_by_id(&self, id: Uuid) -> Result<User, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn fetch_article_content(&self, title: &str) -> Result<Article, sqlx::Error> {
        sqlx::query_as!(
            Article,
            "SELECT * FROM articles WHERE title = $1",
            title
        )
        .fetch_one(&self.pool)
        .await
    }
}
