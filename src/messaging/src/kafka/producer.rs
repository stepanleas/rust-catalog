use kafka::client::{KafkaClient, RequiredAcks};
use kafka::producer::{Producer, Record};
use std::time::Duration;

pub struct KafkaProducer {
    producer: Producer,
}

impl KafkaProducer {
    pub fn new(client: KafkaClient) -> anyhow::Result<Self> {
        let producer = Producer::from_client(client)
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()?;

        Ok(Self { producer })
    }

    pub fn send(&mut self, topic: &str, payload: &[u8]) -> anyhow::Result<()> {
        Ok(self.producer.send(&Record::from_value(topic, payload))?)
    }
}
