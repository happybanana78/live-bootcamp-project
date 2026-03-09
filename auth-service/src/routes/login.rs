use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/login", post(login))
}

async fn login() -> impl IntoResponse {
    StatusCode::OK
}
