use crate::config::Config;
use crate::models::data_store::UserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub user_store: Arc<RwLock<dyn UserStore>>,
    pub config: Config,
}

impl AppState {
    pub fn new(user_store: Arc<RwLock<dyn UserStore>>, config: Config) -> Self {
        Self { user_store, config }
    }
}
