use backend::{app, AppState};

use axum::http::HeaderValue;
use sqlx::sqlite::SqlitePoolOptions;
use std::str::FromStr;

#[tokio::main]
async fn main()
{
    //Ops & Environment
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    //Datenbank-Verbindung aufbauen
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

    sqlx::migrate!("./migrations").run(&pool).await
        .expect("Migrations fehlgeschlagen");

    let state = AppState { db: pool };

    let allowed_origins: Vec<HeaderValue> = std::env::var("ALLOWED_ORIGINS")
    .unwrap_or_default()
    .split(',')
    .filter_map(|s| s.trim().parse::<HeaderValue>().ok())
    .collect();

    if allowed_origins.is_empty() 
    {
        tracing::warn!("ALLOWED_ORIGINS nicht gesetzt – CORS blockiert alle Browser-Requests");
    }

    let router = app(state, allowed_origins, true);
    
    //Netzwerk-Binding
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let server_addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&server_addr)
        .await
        .unwrap();

    tracing::info!("Server running on http://{}", server_addr);

    //Server starten
    axum::serve(listener, router).await.unwrap();
}
