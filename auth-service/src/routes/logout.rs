use crate::auth::jwt::validate_token;
use crate::models::error::AuthAPIError;
use crate::state::AppState;
use crate::utils::constants::JWT_COOKIE_NAME;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;

pub fn routes() -> Router<AppState> {
    Router::new().route("/logout", post(logout))
}

async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value();

    let _ = match validate_token(token).await {
        Ok(claims) => claims,
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
    };

    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));

    (jar, Ok(StatusCode::NO_CONTENT))
}
