use serde::{Dserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
}
