use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/verify-2fa", post(verify_2fa))
}

async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK
}
