use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use sqlx::{
    sqlite::SqlitePoolOptions,
    SqlitePool,
};
use tower::ServiceExt;
use bajam::{models::session::LoginResponse, state::AppState};
use bajam::routes::router::create_router;


#[tokio::test]

// Scenario create user, login, logout (logout use the bearer token)
async fn login_logout() {
    let db = test_db().await;

    let test_state = AppState::new(db).await;
    let test_router = create_router(test_state.unwrap());

    let register_response = test_router.clone().oneshot(
        Request::post("/user/new")
        .header("Content-type", "application/json")
        .body(Body::from(
            r#"{"username": "clemschiste", "password": "secret"}"#
        ))
        .unwrap(),
    )
    .await
    .unwrap();

    // Sous Json({"token": "xxx"})
    let response = test_router.clone().oneshot(
        Request::post("/user/login")
        .header("Content-type", "application/json")
        .body(Body::from(
            r#"{"username": "clemschiste", "password": "secret"}"#
        ))
        .unwrap(),
    )
    .await
    .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
    .await
    .unwrap();

    let login_response: LoginResponse = serde_json::from_slice(&body)
        .unwrap();

    let token = login_response.token;
    println!("session token: {token}");

    let logout_response = test_router.oneshot(
        Request::delete("/user/logout")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(register_response.status(), StatusCode::NO_CONTENT);
    assert_eq!(logout_response.status(), StatusCode::NO_CONTENT);
    
}


pub async fn test_db() -> SqlitePool {
    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::migrate!()
        .run(&db)
        .await
        .unwrap();

    db
}
