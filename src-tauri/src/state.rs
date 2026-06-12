use lp_core::{db::Database, models::AppSettings};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub db: Database,
    pub settings: Arc<RwLock<AppSettings>>,
}

impl AppState {
    pub async fn new(db: Database) -> Self {
        let settings = lp_core::db::queries::load_settings(&db).await.unwrap_or_default();
        Self {
            db,
            settings: Arc::new(RwLock::new(settings)),
        }
    }
}
