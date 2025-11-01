use domain::{ProductCreatedEvent, ProductDeletedEvent, ProductUpdatedEvent};

pub trait ProductMessagePublisher {
    fn publish_created(&self, event: ProductCreatedEvent) -> anyhow::Result<()>;
    fn publish_updated(&self, event: ProductUpdatedEvent) -> anyhow::Result<()>;
    fn publish_deleted(&self, event: ProductDeletedEvent) -> anyhow::Result<()>;
}
