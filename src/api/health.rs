use axum::{response::IntoResponse, Json};
use axum::{routing::get, Router};

pub async fn healthcheck_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}

pub fn create_router() -> Router {
    Router::new().route("/health", get(healthcheck_handler))
}
