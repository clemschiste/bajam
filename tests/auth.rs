use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceExt;
use bajam::state::AppState;
use bajam::routes::router::create_router;


#[tokio::test]
async fn login_works() {
    let test_db = SqlitePoolOptions::new()
    .max_connections(1)
    .connect("sqlite::memory:")
    .await?;

    let test_state = AppState::new(test_db).await;

    let test_router = create_router(test_state);
    
}
