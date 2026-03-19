use apache_avro::Schema;
use serde::Serialize;

pub mod product_created_event_avro_model;
pub mod product_deleted_event_avro_model;
pub mod product_updated_event_avro_model;

fn parse_schema<T: Serialize>(schema_path: &str, value: T) -> anyhow::Result<Vec<u8>> {
    let schema = Schema::parse_str(schema_path)?;
    let value = apache_avro::to_value(value)?;
    let mut writer = apache_avro::Writer::new(&schema, Vec::new());

    writer.append(value)?;
    writer.flush()?;

    Ok(writer.into_inner()?)
}
