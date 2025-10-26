use apache_avro::{Schema, Writer};
use domain::ProductCreatedEvent;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProductCreatedEventAvroModel {
    product: ProductAvroModel,
    created_at: String,
}

#[derive(Serialize)]
struct ProductAvroModel {
    id: String,
    title: String,
    quantity: i32,
    price: String,
}

impl ProductCreatedEventAvroModel {
    pub fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let schema = Schema::parse_str(include_str!(
            "../../../avro/schemas/product_created_event_avro_model.avsc"
        ))
        .expect("invalid Avro schema");

        let value = apache_avro::to_value(self)?;
        let mut writer = Writer::new(&schema, Vec::new());

        writer.append(value)?;
        writer.flush()?;

        Ok(writer.into_inner()?)
    }
}

impl From<ProductCreatedEvent> for ProductCreatedEventAvroModel {
    fn from(event: ProductCreatedEvent) -> Self {
        Self {
            product: ProductAvroModel {
                id: event.product().id().as_uuid().to_string(),
                title: event.product().title().to_string(),
                quantity: event.product().quantity(),
                price: event.product().price().to_string(),
            },
            created_at: event.created_at().to_string(),
        }
    }
}
