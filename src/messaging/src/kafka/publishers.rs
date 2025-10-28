use crate::kafka::avro::{
    AvroSerializable, ProductCreatedEventAvroModel, ProductUpdatedEventAvroModel,
};
use crate::kafka::producer::KafkaProducer;
use anyhow::anyhow;
use application::ProductMessagePublisher;
use domain::{ProductCreatedEvent, ProductUpdatedEvent};
use log::{error, info};
use std::sync::Mutex;

pub struct ProductKafkaMessagePublisher {
    producer: Mutex<KafkaProducer>,
}

impl ProductKafkaMessagePublisher {
    pub fn new(producer: KafkaProducer) -> Self {
        Self {
            producer: Mutex::new(producer),
        }
    }

    fn publish_event<T, E>(&self, topic: &str, event: T) -> anyhow::Result<()>
    where
        T: Into<E>,
        E: AvroSerializable,
    {
        let mut producer = self
            .producer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let avro_model: E = event.into();
        producer.send(topic, &avro_model.to_avro_bytes()?)
    }
}

impl ProductMessagePublisher for ProductKafkaMessagePublisher {
    fn publish_created(&self, event: ProductCreatedEvent) -> anyhow::Result<()> {
        let product_id = &event.product().id().as_uuid().to_string();
        info!(
            "Received ProductCreatedEvent for product with id: {}",
            product_id,
        );

        match self.publish_event::<ProductCreatedEvent, ProductCreatedEventAvroModel>(
            "product-created",
            event,
        ) {
            Ok(_) => {
                info!(
                    "ProductCreatedEvent published successfully for product with id: {}",
                    product_id,
                );

                Ok(())
            }
            Err(error) => {
                error!(
                    "Error while sending ProductCreatedEvent to kafka for product id: {}. {}",
                    product_id,
                    error.to_string(),
                );

                Err(anyhow!(error))
            }
        }
    }

    fn publish_updated(&self, event: ProductUpdatedEvent) -> anyhow::Result<()> {
        let product_id = &event.product().id().as_uuid().to_string();
        info!(
            "Received ProductUpdatedEvent for product with id: {}",
            product_id,
        );

        match self.publish_event::<ProductUpdatedEvent, ProductUpdatedEventAvroModel>(
            "product-updated",
            event,
        ) {
            Ok(_) => {
                info!(
                    "ProductUpdatedEvent published successfully for product with id: {}",
                    product_id,
                );

                Ok(())
            }
            Err(error) => {
                error!(
                    "Error while sending ProductUpdatedEvent to kafka for product id: {}. {}",
                    product_id,
                    error.to_string(),
                );

                Err(anyhow!(error))
            }
        }
    }
}
