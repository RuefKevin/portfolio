use axum::{Json, Router, http::StatusCode, routing::get};
use axum::extract::{Path, State};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::cors::CorsLayer;
use axum::http::{HeaderName, HeaderValue, Method};
use serde::{Serialize};
use sqlx::sqlite::SqlitePool;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::SmartIpKeyExtractor;
use std::sync::Arc;

#[derive(Serialize)]
pub struct Project
{
    pub id: u32,
    pub name: String,
    pub description: String,
    pub technologies: Vec<String>,
}

struct DbProject
{
    id: i64,
    name: String,
    description: String,
    technologies: String,
}

#[derive(Serialize)]
pub struct BlogPost
{
    pub id: u32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published_at: String,
}

struct DbBlogPost
{
    id: Option<i64>,
    title: String,
    slug: String,
    content: String,
    published_at: String,
}

#[derive(Clone)]
pub struct AppState
{
    pub db: SqlitePool,
}

pub fn app(state: AppState, allowed_origins: Vec<HeaderValue>, rate_limit: bool) -> Router
{
    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let router = Router::new()
        .route("/health", get(health))
        .route("/api/projects", get(projects))
        .route("/api/blog", get(blog_posts))
        .route("/api/blog/{slug}", get(blog_post_by_slug))
        .layer(cors)
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

        if rate_limit 
        {
            let governor_config = Arc::new(
                GovernorConfigBuilder::default()
                    .key_extractor(SmartIpKeyExtractor)
                    .per_second(2)
                    .burst_size(5)
                    .finish()
                    .unwrap()
            );
            router.layer(GovernorLayer::new(governor_config))
        } else {
            router
        }
}

async fn health() -> &'static str
{
    "ok"
}

async fn projects(State(state): State<AppState>) -> Result<Json<Vec<Project>>, StatusCode>
{
    let db_projects = sqlx::query_as!
    (
        DbProject,
        "SELECT id, name, description, technologies FROM projects"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {tracing::error!("DB error: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

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

    Ok(Json(projects))
}

async fn blog_posts(State(state): State<AppState>) -> Result<Json<Vec<BlogPost>>, StatusCode>
{
    let db_posts = sqlx::query_as!
    (
        DbBlogPost,
        "SELECT id, title, slug, content, published_at FROM blog_posts"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| { tracing::error!("DB error: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

    let posts: Vec<BlogPost> = db_posts
        .into_iter()
        .map(map_blog_post)
        .collect();

    Ok(Json(posts))
}

async fn blog_post_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<BlogPost>, StatusCode>
{
    let db_post = sqlx::query_as!(
        DbBlogPost,
        "SELECT id, title, slug, content, published_at FROM blog_posts WHERE slug = ?",
        slug
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| { tracing::error!("DB error: {}", e); StatusCode::INTERNAL_SERVER_ERROR })?;

    match db_post
    {
        Some(p) => Ok(Json(map_blog_post(p))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

fn map_blog_post(p: DbBlogPost) -> BlogPost
{
    BlogPost
    {
        id: p.id.unwrap_or(0) as u32,
        title: p.title,
        slug: p.slug,
        content: p.content,
        published_at: p.published_at,
    }
}
