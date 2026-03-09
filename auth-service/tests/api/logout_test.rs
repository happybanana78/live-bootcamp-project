use crate::helpers::TestApp;
use axum::http::StatusCode;

#[tokio::test]
async fn logout() {
    let app = TestApp::new().await;

    let response = app.logout().await;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
