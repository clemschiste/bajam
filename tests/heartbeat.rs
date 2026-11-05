use axum::{
    Router, body::{Body, to_bytes}, http::{Request, Response},
};
use axum::http::StatusCode;

mod common;
use bajam::routes::heartbeat_post::ImageHandle;
use common::*;
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
// Scenario post an heartbeat
pub async fn heartbeat_create() {
    let session = test_init_with_user("user", "secret").await;
    
    let response = test_post_heartbeat(
        session.token,
        session.router
    ).await;

    // Vérifier le status HTTP
    assert_eq!(response.status(), StatusCode::OK);

    // Récupérer le body
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    // Désérialiser le JSON
    let image_handle: ImageHandle =
        serde_json::from_slice(&body).unwrap();

    // Vérifier que l'UUID est valide
    assert_ne!(image_handle.picture_id, Uuid::nil());
}

// Test function for posting a heartbeat
async fn test_post_heartbeat(token: String, router: Router) -> Response<Body> {
    let heartbeat_body = serde_json::json!({
        "latitude": 1.22,
        "longitude": 45.33,
        "description": "tout va bien",
    });

    router.oneshot(
        Request::post("/heartbeat")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .body(Body::from(heartbeat_body.to_string()))
        .unwrap(),
    )
    .await
    .unwrap()
}
