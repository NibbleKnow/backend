pub mod articles;
pub mod auth;
pub mod users;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(auth::routes())
        .merge(users::routes())
        .merge(articles::routes())
}
