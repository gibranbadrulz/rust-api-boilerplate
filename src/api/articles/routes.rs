use crate::api::articles::handler::{get_all_articles, get_article_by_id};
use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new()
        .route("/articles", get(get_all_articles))
        .route("/articles/:id", get(get_article_by_id))
}
