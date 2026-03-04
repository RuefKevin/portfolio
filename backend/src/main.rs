use backend::{app, AppState};

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
    let state = AppState { db: pool };

    let router = app(state);

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
