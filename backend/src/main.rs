use axum::{routing::get, Router};
use tower_http::set_header::SetResponseHeaderLayer;
use axum::http::{HeaderName, HeaderValue};

#[tokio::main]
async fn main()
{
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let app = Router::new()
        .route("/health", get(health))
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
