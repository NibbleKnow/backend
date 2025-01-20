use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: uuid::Uuid,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub author_id: uuid::Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
