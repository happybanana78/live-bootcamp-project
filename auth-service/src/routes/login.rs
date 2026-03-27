use crate::auth::jwt::generate_auth_cookie;
use crate::models::error::AuthAPIError;
use crate::models::user::User;
use crate::requests::login_request::LoginRequest;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use axum_extra::extract::CookieJar;

pub fn routes() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let user = match User::try_from(request) {
        Ok(user) => user,
        Err(err) => return (jar, Err(err)),
    };

    let user_store = &state.user_store.read().await;
    let store_result = user_store.validate_user(user.email.clone(), user.password);

    if store_result.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let auth_cookie = match generate_auth_cookie(&user.email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
}
