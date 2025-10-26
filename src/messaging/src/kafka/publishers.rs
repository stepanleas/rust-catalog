use crate::kafka::avro::ProductCreatedEventAvroModel;
use crate::kafka::producer::KafkaProducer;
use anyhow::anyhow;
use application::ProductMessagePublisher;
use domain::ProductCreatedEvent;
use log::{error, info};
use std::sync::Mutex;

pub struct ProductCreatedEventKafkaPublisher {
    producer: Mutex<KafkaProducer>,
}

impl ProductCreatedEventKafkaPublisher {
    pub fn new(producer: KafkaProducer) -> Self {
        Self {
            producer: Mutex::new(producer),
        }
    }
}

impl ProductMessagePublisher for ProductCreatedEventKafkaPublisher {
    fn publish_created(&self, event: ProductCreatedEvent) -> anyhow::Result<()> {
        let product_id = &event.product().id().as_uuid().to_string();
        info!(
            "Received ProductCreatedEvent for product with id: {}",
            product_id,
        );

        let mut producer = self
            .producer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let avro_model: ProductCreatedEventAvroModel = event.into();

        match producer.send("product-created", &avro_model.to_avro_bytes()?) {
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
}
