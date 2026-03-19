use serde::Serialize;

pub trait AvroSerializable {
    fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>>;
}

#[derive(Serialize)]
pub(crate) struct ProductAvroModel {
    id: String,
    title: String,
    quantity: i32,
    price: String,
}

impl ProductAvroModel {
    pub fn new(id: String, title: String, quantity: i32, price: String) -> Self {
        Self {
            id,
            title,
            quantity,
            price,
        }
    }
}
