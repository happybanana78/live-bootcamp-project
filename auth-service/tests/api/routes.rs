use crate::helpers::TestApp;
use axum::http::StatusCode;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup() {
    let app = TestApp::new().await;

    let payload = serde_json::json!({
        "password": "testpassword",
        "email": "test@example.com",
        "requires2FA": true,
    });

    let response = app.signup(payload).await;

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn login() {
    let app = TestApp::new().await;

    let payload = serde_json::json!({
        "password": "testpassword",
        "email": "test@example.com",
    });

    let response = app.login(payload).await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn logout() {
    let app = TestApp::new().await;

    let response = app.logout().await;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn verify_2fa() {
    let app = TestApp::new().await;

    let payload = serde_json::json!({
        "email": "test@example.com",
        "loginAttemptId": "ciao",
        "2FACode":"12345",
    });

    let response = app.verify_2fa(payload).await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn verify_token() {
    let app = TestApp::new().await;

    let payload = serde_json::json!({
        "token": "testpassword",
    });

    let response = app.verify_token(payload).await;

    assert_eq!(response.status(), StatusCode::OK);
}
