use uuid::Uuid;


#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: u64,
}


#[derive(Debug)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: u64,
}
