use application::{CategoryRepository, ProductRepository, Settings};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub category_repository: Arc<dyn CategoryRepository + Send + Sync>,
    pub product_repository: Arc<dyn ProductRepository + Send + Sync>,
}
