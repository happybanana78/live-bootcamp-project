pub mod auth;
pub mod config;
pub mod models;
pub mod requests;
pub mod routes;
pub mod services;
pub mod state;
pub mod utils;

use crate::models::error::AuthAPIError;
use crate::state::AppState;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::serve::Serve;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let assets_dir =
            ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

        let router = Router::new()
            .fallback_service(assets_dir)
            .merge(routes::build_routes())
            .with_state(app_state);

        let listener = TcpListener::bind(address).await?;

        let address = listener.local_addr()?.to_string();

        let server = axum::serve(listener, router);

        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Unauthorized"),
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}
