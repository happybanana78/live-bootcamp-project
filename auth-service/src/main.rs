use auth_service::config::Config;
use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::state::AppState;
use auth_service::Application;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));

    let config = Config::from_env();

    let app_state = AppState::new(user_store, config);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
