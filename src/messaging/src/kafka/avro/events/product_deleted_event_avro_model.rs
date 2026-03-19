use crate::kafka::avro::events::parse_schema;
use crate::kafka::avro::models::AvroSerializable;
use domain::events::ProductDeletedEvent;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProductDeletedEventAvroModel {
    id: String,
    product_id: String,
    created_at: String,
}

impl AvroSerializable for ProductDeletedEventAvroModel {
    fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let schema_path =
            include_str!("../../../../avro/schemas/product_deleted_event_avro_model.avsc");

        parse_schema(schema_path, self)
    }
}

impl From<ProductDeletedEvent> for ProductDeletedEventAvroModel {
    fn from(event: ProductDeletedEvent) -> Self {
        Self {
            id: event.id().into(),
            product_id: event.product_id().as_uuid().to_string(),
            created_at: event.created_at().to_string(),
        }
    }
}
