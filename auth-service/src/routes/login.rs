use crate::models::error::AuthAPIError;
use crate::models::user::User;
use crate::requests::login_request::LoginRequest;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

pub fn routes() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let user = User::try_from(request)?;

    let user_store = &state.user_store.read().await;
    user_store
        .validate_user(user.email, user.password)
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    Ok(StatusCode::OK)
}
