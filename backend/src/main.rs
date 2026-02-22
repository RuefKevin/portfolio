use axum::{routing::get, Router};
use tower_http::set_header::SetResponseHeaderLayer;
use axum::http::{HeaderName, HeaderValue};
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Project
{
    id: u32,
    name: String,
    description: String,
    technologies: Vec<String>,
}

#[tokio::main]
async fn main()
{
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/projects", get(projects))
        .layer(SetResponseHeaderLayer::overriding
        (
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding
        (
            HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str
{
    "ok"
}

async fn projects() -> Json<Vec<Project>>
{
    Json
    (
        vec!
        [
            Project
            {
                id: 1,
                name: "Portfolio Backend".to_string(),
                description: "Rust/Axum REST API with security focus".to_string(),
                technologies: vec!["Rust".to_string(), "Axum".to_string(), "SQLite".to_string()],
            }
        ]
    )
}
