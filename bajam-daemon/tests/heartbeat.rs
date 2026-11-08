use axum::{
    Router, body::{Body, to_bytes}, http::{Request, Response},
};
use axum::http::StatusCode;

mod common;
use bajam_daemon::routes::{escapade_post::EscapadeResponse, heartbeat_post::ImageHandle};
use common::*;
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
// Scenario post an heartbeat
pub async fn heartbeat_create() {
    let session = test_init_with_user("user", "secret").await;

    // Creation of an escapade first, getting the id
    let escapade_response = test_post_escapade(
        session.token.clone(),
        session.router.clone(),
        "Japon").await;

    assert_eq!(escapade_response.status(), StatusCode::OK);

    let escapade_id = test_extract_escapade_id(escapade_response).await;

    // Creation of the heartbeat, inserting the escapade_id
    let heartbeat_response = test_post_heartbeat(
        session.token.clone(),
        session.router.clone(),
        escapade_id
    ).await;

    // Vérifier le status HTTP
    assert_eq!(heartbeat_response.status(), StatusCode::OK);

    // Récupérer le body
    let body = to_bytes(heartbeat_response.into_body(), usize::MAX)
        .await
        .unwrap();

    // Désérialiser le JSON
    let image_handle: ImageHandle =
        serde_json::from_slice(&body).unwrap();

    // Vérifier que l'UUID est valide
    assert_ne!(image_handle.picture_id, Uuid::nil());

    // Suppression de l'escapade
    let escapade_deletion_response = test_delete_ascapade(session.token, session.router, escapade_id).await;

    assert_eq!(escapade_deletion_response.status(), StatusCode::NO_CONTENT);
    
}

// Test function for posting a heartbeat
async fn test_post_heartbeat(token: String, router: Router, escapade_id: i64) -> Response<Body> {
    let heartbeat_body = serde_json::json!({
        "latitude": 1.22,
        "longitude": 45.33,
        "description": "tout va bien",
    });

    router.oneshot(
        Request::post(format!("/escapades/{}/heartbeat", escapade_id))
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .body(Body::from(heartbeat_body.to_string()))
        .unwrap(),
    )
    .await
    .unwrap()
}

// Test function for posting an escapade
async fn test_post_escapade(token: String, router: Router, name: &str) -> Response<Body> {
    let escapade_body = serde_json::json!({
        "name": name,
    });

    router.oneshot(
        Request::post("/escapades")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .body(Body::from(escapade_body.to_string()))
        .unwrap(),
    )
    .await
    .unwrap()
}

pub async fn test_extract_escapade_id(escapade_response: Response<Body>) -> i64 {
    let escapade_body = axum::body::to_bytes(escapade_response.into_body(), usize::MAX)
        .await.unwrap();
    let id: EscapadeResponse = serde_json::from_slice(&escapade_body).unwrap(); 
    id.escapade_id
}

async fn test_delete_ascapade(token: String, router: Router, escapade_id: i64) -> Response<Body> {
    router.oneshot(
        Request::delete(format!("/escapades/{}", escapade_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap()
    )
    .await
    .unwrap()
}

// todo -> test deleting heartbeat and unauthorized deleting of heartbeat
