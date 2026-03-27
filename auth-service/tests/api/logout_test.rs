use crate::helpers::{get_random_email, TestApp};
use auth_service::utils::constants::JWT_COOKIE_NAME;
use axum::http::StatusCode;
use reqwest::Url;

#[tokio::test]
async fn logout() {
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
        "password": "test_password",
        "email": email,
    });

    let login_response = app.login(login_payload).await;

    let auth_cookie = login_response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME,
            auth_cookie.value()
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let logout_response = app.logout().await;

    assert_eq!(logout_response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
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
        "password": "test_password",
        "email": email,
    });

    let login_response = app.login(login_payload).await;

    let auth_cookie = login_response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME,
            auth_cookie.value()
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let logout_response_1 = app.logout().await;

    assert_eq!(logout_response_1.status(), StatusCode::NO_CONTENT);

    let logout_response_2 = app.logout().await;

    assert_eq!(logout_response_2.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.logout().await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.logout().await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
