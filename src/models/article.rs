use serde::{Deserialize, Searialize};
use sqlx::FromRow;
use time::OffsetDatetime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: uuid,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub author_id: uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
