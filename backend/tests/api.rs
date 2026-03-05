use backend::{app, AppState, Project};

use axum::
{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use sqlx::sqlite::SqlitePool;
use tower::ServiceExt;

fn test_app(pool: SqlitePool) -> Router 
{
    app(AppState { db: pool }, vec![])
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

    sqlx::query!("DELETE FROM projects")
        .execute(&pool)
        .await
        .expect("Failed to clear projects table");

    sqlx::query!("INSERT INTO projects (name, description, technologies) VALUES ('Sec-Portfolio', 'Test desc', 'Rust, Svelte')")
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
    let projects: Vec<Project> = serde_json::from_slice(&body).unwrap();

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "Sec-Portfolio");
    assert_eq!(projects[0].technologies, vec!["Rust", "Svelte"]);
}
