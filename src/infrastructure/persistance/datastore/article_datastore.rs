use crate::domain::model::article::{Article, ArticleBuilder};
use crate::domain::repository::article_repository::ArticleRepository;
use axum::async_trait;
use sqlx::postgres::PgPool;

pub struct SqlxArticleRepository {
    pub pool: PgPool,
}

#[async_trait]
impl ArticleRepository for SqlxArticleRepository {
    async fn find_all(&self) -> Result<Vec<Article>, sqlx::Error> {
        let articles = sqlx::query_as!(
            Article,
            "SELECT id, title, slug, thumbnail, image, description, body FROM article"
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|record| {
            ArticleBuilder::default()
                .id(record.id)
                .title(record.title)
                .slug(record.slug)
                .thumbnail(record.thumbnail)
                .image(record.image)
                .description(record.description)
                .body(record.body)
                .build()
        })
        .collect();

        Ok(articles)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Article>, sqlx::Error> {
        let article = sqlx::query_as!(
            Article,
            "SELECT id, title, slug, thumbnail, image, description, body FROM article WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(article)
    }
}
