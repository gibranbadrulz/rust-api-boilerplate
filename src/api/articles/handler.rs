use crate::domain::model::article::Article;
use crate::domain::service::article_service::ArticleServiceImpl;
use axum::{extract::Extension, extract::Path, Json};
use std::sync::Arc;

pub async fn get_all_articles(
    Extension(service): Extension<Arc<ArticleServiceImpl>>,
) -> Json<Vec<Article>> {
    let articles = service.get_all_articles().await.unwrap();
    Json(articles)
}

pub async fn get_article_by_id(
    Extension(service): Extension<Arc<ArticleServiceImpl>>,
    Path(id): Path<i32>,
) -> Json<Option<Article>> {
    let article = service.get_article_by_id(id).await.unwrap();
    Json(article)
}
