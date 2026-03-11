use crate::helpers::{get_random_email, TestApp};
use auth_service::routes::signup::SignupResponse;
use auth_service::ErrorResponse;
use axum::http::StatusCode;

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let payload = serde_json::json!({
        "password": "testpassword",
        "email": get_random_email(),
        "requires2FA": true,
    });

    let response = app.signup(payload).await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "password123",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.signup(test_case).await;

        assert_eq!(
            response.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let input = [
        serde_json::json!({
            "email": "test@example.com",
            "password": "short",
            "requires2FA": true,
        }),
        serde_json::json!({
            "email": "invalid_email",
            "password": "123456789",
            "requires2FA": false,
        }),
    ];

    for i in input.iter() {
        let response = app.signup(i).await;

        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "Failed for input: {:?}",
            i
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;

    let email = get_random_email();

    let payload = serde_json::json!({
        "password": "testpassword",
        "email": email,
        "requires2FA": true,
    });

    app.signup(payload.clone()).await;
    let response = app.signup(payload).await;

    assert_eq!(response.status(), StatusCode::CONFLICT);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}
