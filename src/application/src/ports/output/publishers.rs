use domain::events::{ProductCreatedEvent, ProductDeletedEvent, ProductUpdatedEvent};
use mockall::automock;

#[automock]
pub trait ProductMessagePublisher {
    fn publish_created(&self, event: ProductCreatedEvent) -> anyhow::Result<()>;
    fn publish_updated(&self, event: ProductUpdatedEvent) -> anyhow::Result<()>;
    fn publish_deleted(&self, event: ProductDeletedEvent) -> anyhow::Result<()>;
}
