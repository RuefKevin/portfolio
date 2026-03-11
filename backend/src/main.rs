use backend::{app, AppState};

use axum::http::HeaderValue;
use sqlx::sqlite::SqlitePoolOptions;
use std::str::FromStr;

#[tokio::main]
async fn main()
{
    // Initialize tracing and environment
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    // Connect to database
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect_with(
        sqlx::sqlite::SqliteConnectOptions::from_str(&db_url)
            .unwrap_or_else(|e| panic!("Invalid DB URL: {}", e))
            .create_if_missing(true)
    )
    .await
    .unwrap_or_else(|e| panic!("DB connect failed: {}", e));

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await
        .expect("Migrations fehlgeschlagen");

    let state = AppState { db: pool };

    // Parse allowed CORS origins
    let allowed_origins: Vec<HeaderValue> = std::env::var("ALLOWED_ORIGINS")
    .unwrap_or_default()
    .split(',')
    .filter_map(|s| s.trim().parse::<HeaderValue>().ok())
    .collect();

    if allowed_origins.is_empty() 
    {
        tracing::warn!("ALLOWED_ORIGINS nicht gesetzt – CORS blockiert alle Browser-Requests");
    }

    // Rate limiting (disabled locally, enable via RATE_LIMIT=true)
    let rate_limit = std::env::var("RATE_LIMIT")
        .unwrap_or_else(|_| "true".to_string()) == "true";

    let router = app(state, allowed_origins, rate_limit);
    
    // Bind to network
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let server_addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&server_addr)
        .await
        .unwrap();

    tracing::info!("Server running on http://{}", server_addr);

    //Start server
    axum::serve(listener, router).await.unwrap();
}
