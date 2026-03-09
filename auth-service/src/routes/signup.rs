use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/signup", post(signup))
}

async fn signup() -> impl IntoResponse {
    StatusCode::CREATED
}
