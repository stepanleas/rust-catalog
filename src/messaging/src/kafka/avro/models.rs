use apache_avro::Schema;
use domain::{ProductCreatedEvent, ProductDeletedEvent, ProductUpdatedEvent};
use serde::Serialize;

pub trait AvroSerializable {
    fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>>;
}

#[derive(Serialize)]
struct ProductAvroModel {
    id: String,
    title: String,
    quantity: i32,
    price: String,
}

#[derive(Serialize)]
pub struct ProductCreatedEventAvroModel {
    id: String,
    product: ProductAvroModel,
    created_at: String,
}

impl AvroSerializable for ProductCreatedEventAvroModel {
    fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let schema_path =
            include_str!("../../../avro/schemas/product_created_event_avro_model.avsc");

        parse_schema(schema_path, self)
    }
}

impl From<ProductCreatedEvent> for ProductCreatedEventAvroModel {
    fn from(event: ProductCreatedEvent) -> Self {
        Self {
            id: event.id().into(),
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

#[derive(Serialize)]
pub struct ProductUpdatedEventAvroModel {
    id: String,
    product: ProductAvroModel,
    created_at: String,
}

impl AvroSerializable for ProductUpdatedEventAvroModel {
    fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let schema_path =
            include_str!("../../../avro/schemas/product_updated_event_avro_model.avsc");

        parse_schema(schema_path, self)
    }
}

impl From<ProductUpdatedEvent> for ProductUpdatedEventAvroModel {
    fn from(event: ProductUpdatedEvent) -> Self {
        Self {
            id: event.id().into(),
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

#[derive(Serialize)]
pub struct ProductDeletedEventAvroModel {
    id: String,
    product_id: String,
    created_at: String,
}

impl AvroSerializable for ProductDeletedEventAvroModel {
    fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let schema_path =
            include_str!("../../../avro/schemas/product_deleted_event_avro_model.avsc");

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

fn parse_schema<T: Serialize>(schema_path: &str, value: T) -> anyhow::Result<Vec<u8>> {
    let schema = Schema::parse_str(schema_path)?;
    let value = apache_avro::to_value(value)?;
    let mut writer = apache_avro::Writer::new(&schema, Vec::new());

    writer.append(value)?;
    writer.flush()?;

    Ok(writer.into_inner()?)
}
