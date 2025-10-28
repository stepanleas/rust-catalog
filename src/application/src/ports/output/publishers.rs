use domain::{ProductCreatedEvent, ProductUpdatedEvent};

pub trait ProductMessagePublisher {
    fn publish_created(&self, event: ProductCreatedEvent) -> anyhow::Result<()>;
    fn publish_updated(&self, event: ProductUpdatedEvent) -> anyhow::Result<()>;
}
