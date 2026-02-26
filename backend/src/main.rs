use axum::{extract::State, routing::get, Router, Json};
use tower_http::set_header::SetResponseHeaderLayer;
use axum::http::{HeaderName, HeaderValue};
use serde::Serialize;
use sqlx::sqlite::SqlitePool;

#[derive(Serialize)]
struct Project
{
    id: u32,
    name: String,
    description: String,
    technologies: Vec<String>,
}

struct DbProject
{
    id: i64,
    name: String,
    description: String,
    technologies: String,
}

#[derive(Clone)]
struct AppState
{
    db: SqlitePool,
}

#[tokio::main]
async fn main()
{
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = SqlitePool::connect(&db_url).await.unwrap();
    let state = AppState { db: pool };

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
        ))
        .with_state(state);

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let server_addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&server_addr)
        .await
        .unwrap();

    tracing::info!("Server running on http://{}", server_addr);
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str
{
    "ok"
}

async fn projects(State(state): State<AppState>) -> Json<Vec<Project>>
{
    let db_projects = sqlx::query_as!
    (
        DbProject,
        "SELECT id, name, description, technologies FROM projects"
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    let projects: Vec<Project> = db_projects
        .into_iter()
        .map(|p| Project
        {
            id: p.id as u32,
            name: p.name,
            description: p.description,
            technologies: p.technologies.split(',').map(|s| s.trim().to_string()).collect(),
        })
        .collect();

    Json(projects)
}
