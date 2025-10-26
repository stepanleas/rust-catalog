use domain::ProductCreatedEvent;

pub trait ProductMessagePublisher {
    fn publish_created(&self, event: ProductCreatedEvent) -> anyhow::Result<()>;
}
