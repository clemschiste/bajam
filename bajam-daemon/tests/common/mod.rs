use axum::{
    Router, body::Body, http::{Request, Response},
};
use sqlx::{
    sqlite::SqlitePoolOptions,
    SqlitePool,
};
use tower::ServiceExt;
use bajam_daemon::{models::session::LoginResponse};
use bajam_daemon::state::AppState;
use bajam_daemon::routes::router::create_router;

#[allow(dead_code)]
pub struct Session {
    pub token: String,
    pub router: Router,
}

#[allow(dead_code)]
pub async fn test_init_with_user(username: &str, password: &str) -> Session {
    let db = test_db().await;
    let test_state = AppState::new(db).await;
    let test_router = create_router(test_state.unwrap());
    test_register_user(username, password, test_router.clone()).await;
    let response = test_login_user(username, password, test_router.clone()).await;

    Session {
        token : test_extract_token_on_login(response).await,
        router: test_router 
    }
}

// Test function for registering a user, output the response
pub async fn test_register_user(username: &str, password: &str, router: Router) -> Response<Body> {
    let register_body = serde_json::json!({
        "username": username,
        "password": password,
    });
    
    router.oneshot(
        Request::post("/user/new")
        .header("Content-type", "application/json")
        .body(Body::from(register_body.to_string()))
        .unwrap(),
    )
    .await
    .unwrap()
}

// Test function for loging a registered user, outputs the response
pub async fn test_login_user(username: &str, password: &str, router: Router) -> Response<Body> {
    let login_body = serde_json::json!({
        "username": username,
        "password": password,
    });

    router.oneshot(
        Request::post("/user/login")
        .header("Content-type", "application/json")
        .body(Body::from(login_body.to_string()))
        .unwrap(),
    )
    .await
    .unwrap()
}

// panics if error variant
pub async fn test_extract_token_on_login(login_response: Response<Body>) -> String {
    let login_body = axum::body::to_bytes(login_response.into_body(), usize::MAX)
        .await.unwrap();
    let log: LoginResponse = serde_json::from_slice(&login_body).unwrap(); 
    log.token
}

#[allow(dead_code)]
// Test function for logging out a user, output the response
pub async fn test_logout_user(token: &str, router: Router) -> Response<Body> {
    router.oneshot(
        Request::delete("/user/logout")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap(),
    )
    .await
    .unwrap()
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
