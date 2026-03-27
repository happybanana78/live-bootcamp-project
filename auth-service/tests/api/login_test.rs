use crate::helpers::{get_random_email, TestApp};
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

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let payloads = vec![
        serde_json::json!({
            "email": "test@example.com",
        }),
        serde_json::json!({
            "password": "testpassword",
        }),
    ];

    for p in payloads {
        let response = app.login(p).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let payloads = vec![
        serde_json::json!({
            "password": "short",
            "email": "test@example.com",
        }),
        serde_json::json!({
            "password": "testpassword",
            "email": "testexample.com",
        }),
    ];

    for p in payloads {
        let response = app.login(p).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let email = get_random_email();

    let signup_payload = serde_json::json!({
        "password": "test_password",
        "email": email,
        "requires2FA": false,
    });

    let signup_response = app.signup(signup_payload).await;

    assert_eq!(signup_response.status(), StatusCode::CREATED);

    let login_payload = serde_json::json!({
        "password": "wrong_password",
        "email": email,
    });

    let login_response = app.login(login_payload).await;

    assert_eq!(login_response.status(), StatusCode::UNAUTHORIZED);
}
