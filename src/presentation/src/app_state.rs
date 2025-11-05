use application::category::repositories::CategoryRepository;
use application::ports::output::publishers::ProductMessagePublisher;
use application::product::repositories::ProductRepository;
use application::settings::Settings;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub category_repository: Arc<dyn CategoryRepository + Send + Sync>,
    pub product_repository: Arc<dyn ProductRepository + Send + Sync>,
    pub product_message_publisher: Arc<dyn ProductMessagePublisher + Send + Sync>,
}
