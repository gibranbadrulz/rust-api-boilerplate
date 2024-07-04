use crate::domain::model::article::Article;
use axum::async_trait;

#[async_trait]
pub trait ArticleRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Article>, sqlx::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Article>, sqlx::Error>;
}
