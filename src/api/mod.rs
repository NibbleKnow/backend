pub mod article;
pub mod user;


use axum::Router;
use crate::AppState;


pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(user::routes())
        .merge(article::routes())
}
