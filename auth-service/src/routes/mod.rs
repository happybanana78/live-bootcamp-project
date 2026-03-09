use axum::Router;

pub mod login;
pub mod logout;
pub mod signup;
pub mod verify_2fa;
pub mod verify_token;

pub fn build_routes() -> Router {
    Router::new()
        .merge(login::routes())
        .merge(logout::routes())
        .merge(signup::routes())
        .merge(verify_2fa::routes())
        .merge(verify_token::routes())
}
