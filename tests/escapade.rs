use axum::{
    Router, body::{Body}, http::{Request, Response},
};
use axum::http::StatusCode;

mod common;
use common::*;
use tower::ServiceExt;

#[tokio::test]
// Scenario post an escapade
pub async fn escapade_create() {
    let session = test_init_with_user("user", "secret").await;
    
    let response = test_post_escapade(
        session.token,
        session.router
    ).await;

    // Vérifier le status HTTP
    assert_eq!(response.status(), StatusCode::CREATED);
}

// Test function for posting an escapade as a user
async fn test_post_escapade(token: String, router: Router) -> Response<Body> {
    let escapade_body = serde_json::json!({
        "name": "Japon",
    });

    router.oneshot(
        Request::post("/escapade")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .body(Body::from(escapade_body.to_string()))
        .unwrap(),
    )
    .await
    .unwrap()
}
