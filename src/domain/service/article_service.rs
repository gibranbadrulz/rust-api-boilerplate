use crate::domain::model::article::Article;
use crate::domain::repository::article_repository::ArticleRepository;
use std::sync::Arc;

pub struct ArticleServiceImpl {
    repository: Arc<dyn ArticleRepository>,
}

impl ArticleServiceImpl {
    pub fn new(repository: Arc<dyn ArticleRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_articles(&self) -> Result<Vec<Article>, sqlx::Error> {
        self.repository.find_all().await
    }

    pub async fn get_article_by_id(&self, id: i32) -> Result<Option<Article>, sqlx::Error> {
        self.repository.find_by_id(id).await
    }
}
