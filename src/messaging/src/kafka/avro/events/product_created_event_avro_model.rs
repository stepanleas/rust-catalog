use crate::kafka::avro::events::parse_schema;
use crate::kafka::avro::models::{AvroSerializable, ProductAvroModel};
use domain::events::ProductCreatedEvent;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProductCreatedEventAvroModel {
    id: String,
    product: ProductAvroModel,
    created_at: String,
}

impl AvroSerializable for ProductCreatedEventAvroModel {
    fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let schema_path =
            include_str!("../../../../avro/schemas/product_created_event_avro_model.avsc");

        parse_schema(schema_path, self)
    }
}

impl From<ProductCreatedEvent> for ProductCreatedEventAvroModel {
    fn from(event: ProductCreatedEvent) -> Self {
        Self {
            id: event.id().into(),
            product: ProductAvroModel::new(
                event.product().id().as_uuid().to_string(),
                event.product().title().to_string(),
                event.product().quantity(),
                event.product().price().to_string(),
            ),
            created_at: event.created_at().to_string(),
        }
    }
}
