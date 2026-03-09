use crate::helpers::TestApp;
use axum::http::StatusCode;

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
