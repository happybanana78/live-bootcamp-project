use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/verify-token", post(verify_token))
}

async fn verify_token() -> impl IntoResponse {
    StatusCode::OK
}
