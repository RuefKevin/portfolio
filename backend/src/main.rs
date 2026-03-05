use backend::{app, AppState};

use axum::http::HeaderValue;
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main()
{
    //Ops & Environment
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    //Datenbank-Verbindung aufbauen
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = SqlitePool::connect(&db_url).await.unwrap();
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

    let router = app(state, allowed_origins);

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
