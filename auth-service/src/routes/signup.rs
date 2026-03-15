use crate::models::data_store::UserStore;
use crate::models::error::AuthAPIError;
use crate::models::user::User;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

pub fn routes() -> Router<AppState> {
    Router::new().route("/signup", post(signup))
}

async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let user = User::try_from(request)?;

    let mut user_store = state.user_store.write().await;

    if user_store.get_user(&user.email).is_err() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    user_store
        .add_user(user)
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
