use crate::helpers::TestApp;
use axum::http::StatusCode;

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
