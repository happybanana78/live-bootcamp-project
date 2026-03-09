use crate::helpers::TestApp;
use axum::http::StatusCode;

#[tokio::test]
async fn verify_token() {
    let app = TestApp::new().await;

    let payload = serde_json::json!({
        "token": "testpassword",
    });

    let response = app.verify_token(payload).await;

    assert_eq!(response.status(), StatusCode::OK);
}
