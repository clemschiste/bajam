use axum::http::StatusCode;
use bajam::state::AppState;
use bajam::routes::router::create_router;

mod common;
use common::*;


#[tokio::test]
// Scenario create user, login, logout
pub async fn login_logout() {
    let db = test_db().await;
    let test_state = AppState::new(db).await;
    let test_router = create_router(test_state.unwrap());

    let register_response = test_register_user("user", "secret", test_router.clone()).await;
    let login_response = test_login_user("user", "secret", test_router.clone()).await; 
    let token = test_extract_token_on_login(login_response).await;
    println!("session token: {token}");

    let logout_response = test_logout_user(&token, test_router.clone()).await;

    assert_eq!(register_response.status(), StatusCode::NO_CONTENT);
    assert_eq!(logout_response.status(), StatusCode::NO_CONTENT);
}
 
#[tokio::test]
// Scenario create user, wrong login
pub async fn register_and_wrong_login() {
    let db = test_db().await;
    let test_state = AppState::new(db).await;
    let test_router = create_router(test_state.unwrap());

    let register_response = test_register_user("user", "secret", test_router.clone()).await;
    let login_response = test_login_user("user", "wrong_secret", test_router.clone()).await;

    assert_eq!(register_response.status(), StatusCode::NO_CONTENT);
    assert_eq!(login_response.status(), StatusCode::UNAUTHORIZED);
}

