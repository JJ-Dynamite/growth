use axum::{routing::get, Router, Json, response::IntoResponse};
use tower_http::cors::{CorsLayer, Any};

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({"status": "healthy", "service": "growth"}))
}

async fn root() -> impl IntoResponse {
    Json(serde_json::json!({"service": "growth", "version": "0.1.0", "description": "UX teardowns in comic form"}))
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
