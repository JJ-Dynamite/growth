use axum::{
    routing::get,
    Router, Json, response::IntoResponse,
};
use tower_http::cors::{CorsLayer, Any};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HealthResponse { status: String, service: String, version: String }
#[derive(Serialize)]
struct RootResponse { service: String, version: String, description: String, endpoints: Vec<String> }
#[derive(Serialize)]
struct Teardown { id: String, title: String, company: String, summary: String, thumbnail: String, panels: Vec<Panel>, tags: Vec<String> }
#[derive(Serialize)]
struct Panel { panel_number: u32, image_url: String, caption: String }
#[derive(Serialize)]
struct TeardownListResponse { teardowns: Vec<TeardownSummary>, total: usize }
#[derive(Serialize)]
struct TeardownSummary { id: String, title: String, company: String, summary: String, thumbnail: String, tags: Vec<String> }

async fn health() -> impl IntoResponse {
    Json(HealthResponse { status: "healthy".into(), service: "growth".into(), version: "0.1.0".into() })
}

async fn root() -> impl IntoResponse {
    Json(RootResponse {
        service: "growth".into(), version: "0.1.0".into(),
        description: "UX teardowns in comic form".into(),
        endpoints: vec!["GET /health".into(), "GET /teardowns".into(), "GET /teardowns/:id".into()],
    })
}

async fn list_teardowns() -> impl IntoResponse {
    let teardowns = vec![
        TeardownSummary { id: "1".into(), title: "Stripe Checkout Flow".into(), company: "Stripe".into(), summary: "How Stripe simplified payments".into(), thumbnail: "/teardowns/1/thumb.jpg".into(), tags: vec!["checkout".into(), "payments".into(), "simplification".into()] },
        TeardownSummary { id: "2".into(), title: "Linear Onboarding".into(), company: "Linear".into(), summary: "The art of zero-config onboarding".into(), thumbnail: "/teardowns/2/thumb.jpg".into(), tags: vec!["onboarding".into(), "developer-tools".into()] },
    ];
    Json(TeardownListResponse { teardowns, total: teardowns.len() })
}

async fn get_teardown(axum::extract::Path(id): axum::extract::Path<String>) -> impl IntoResponse {
    let panels = vec![
        Panel { panel_number: 1, image_url: format!("/teardowns/{}/p1.jpg", id), caption: "The landing page grabs attention immediately.".into() },
        Panel { panel_number: 2, image_url: format!("/teardowns/{}/p2.jpg", id), caption: "The signup flow is frictionless.".into() },
        Panel { panel_number: 3, image_url: format!("/teardowns/{}/p3.jpg", id), caption: "Users get value within seconds.".into() },
    ];
    Json(Teardown {
        id: id.clone(), title: "UX Teardown".into(), company: "Company".into(),
        summary: "A detailed UX analysis".into(), thumbnail: format!("/teardowns/{}/thumb.jpg", id),
        panels, tags: vec!["ux".into(), "analysis".into()],
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let app = Router::new()
        .route("/", get(root)).route("/health", get(health))
        .route("/teardowns", get(list_teardowns)).route("/teardowns/:id", get(get_teardown))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("growth backend running on port 3001");
    axum::serve(listener, app).await.unwrap();
}
