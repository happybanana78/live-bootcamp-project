use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/logout", post(logout))
}

async fn logout() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}
