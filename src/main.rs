use axum::Router;
use sqlx::postgres::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod api;
mod config;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    let config = config::AppConfig::from_env();

    let pool: PgPool = match config.create_pg_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create pool: {}", e);
            return;
        }
    };

    let repo = Arc::new(
        infrastructure::persistance::datastore::article_datastore::SqlxArticleRepository { pool },
    );
    let service = Arc::new(domain::service::article_service::ArticleServiceImpl::new(
        repo,
    ));

    let app = Router::new()
        .merge(api::health::create_router())
        .merge(api::articles::routes::create_router())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(axum::extract::Extension(service))
                .into_inner(),
        );

    let addr = SocketAddr::from((
        config.app_host.parse::<std::net::IpAddr>().unwrap(),
        config.app_port,
    ));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!(
        "🚀 [{}] listening on {}",
        config.app_env,
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
