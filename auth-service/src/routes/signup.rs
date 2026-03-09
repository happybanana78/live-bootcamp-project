use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new().route("/signup", post(signup))
}

async fn signup(Json(request): Json<SignupRequest>) -> impl IntoResponse {
    StatusCode::CREATED
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
