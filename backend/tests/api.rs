use backend::{app, AppState};

use axum::
{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use sqlx::sqlite::SqlitePool;
use tower::ServiceExt;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct TestProject {
    id: u32,
    name: String,
    description: String,
    technologies: Vec<String>,
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct TestBlogPost {
    id: u32,
    title: String,
    slug: String,
    content: String,
    published_at: String,
}

fn test_app(pool: SqlitePool) -> Router 
{
    app(AppState { db: pool }, vec![], false)
}

async fn setup_test_db() -> SqlitePool
{
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate test database");

    pool
}

#[tokio::test]
async fn test_health_check()
{
    let pool = setup_test_db().await;
    let router = test_app(pool);

    let response = router
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_security_headers_are_present()
{
    let pool = setup_test_db().await;
    let router = test_app(pool);

    let response = router
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.headers().get("x-frame-options").unwrap(), "DENY");
}

#[tokio::test]
async fn test_get_projects_returns_db_data()
{
    let pool = setup_test_db().await;

    sqlx::query("DELETE FROM projects")
        .execute(&pool)
        .await
        .expect("Failed to clear projects table");

    sqlx::query("INSERT INTO projects (name, description, technologies) VALUES ('Sec-Portfolio', 'Test desc', 'Rust, Svelte')")
        .execute(&pool)
        .await
        .expect("Failed to insert test project");
    let router = test_app(pool);

    let response = router
        .oneshot(Request::builder().uri("/api/projects").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let projects: Vec<TestProject> = serde_json::from_slice(&body).unwrap();

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "Sec-Portfolio");
    assert_eq!(projects[0].technologies, vec!["Rust", "Svelte"]);
}

#[tokio::test]
async fn test_get_blog_posts_returns_db_data()
{
    let pool = setup_test_db().await;

    sqlx::query("DELETE FROM blog_posts")
        .execute(&pool)
        .await
        .expect("Failed to clear blog_posts table");

    sqlx::query("INSERT INTO blog_posts (title, slug, content, published_at) VALUES ('Test Post', 'test-post', 'Test content', '2026-03-10')")
        .execute(&pool)
        .await
        .expect("Failed to insert test blog post");

    let router = test_app(pool);

    let response = router
        .oneshot(Request::builder().uri("/api/blog").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let posts: Vec<TestBlogPost> = serde_json::from_slice(&body).unwrap();

    assert_eq!(posts.len(), 1);
    assert_eq!(posts[0].title, "Test Post");
    assert_eq!(posts[0].slug, "test-post");
}

#[tokio::test]
async fn test_get_blog_post_by_slug()
{
    let pool = setup_test_db().await;

    sqlx::query("DELETE FROM blog_posts")
        .execute(&pool)
        .await
        .expect("Failed to clear blog_posts table");

    sqlx::query("INSERT INTO blog_posts (title, slug, content, published_at) VALUES ('Test Post', 'test-post', 'Test content', '2026-03-10')")
        .execute(&pool)
        .await
        .expect("Failed to insert test blog post");

    let router = test_app(pool);

    let response = router
        .oneshot(Request::builder().uri("/api/blog/test-post").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let post: TestBlogPost = serde_json::from_slice(&body).unwrap();

    assert_eq!(post.title, "Test Post");
    assert_eq!(post.slug, "test-post");
}

#[tokio::test]
async fn test_get_blog_post_by_slug_not_found()
{
    let pool = setup_test_db().await;
    let router = test_app(pool);

    let response = router
        .oneshot(Request::builder().uri("/api/blog/nicht-vorhanden").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}