use crate::helpers::TestApp;
use axum::http::StatusCode;

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
