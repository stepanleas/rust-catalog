use crate::responses::category_responses::CategoryResponse;
use application::product::dtos::ProductDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[readonly::make]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProductResponse {
    id: Uuid,
    title: String,
    description: String,
    quantity: i32,
    price: String,
    category: CategoryResponse,
}

impl From<ProductDto> for ProductResponse {
    fn from(product: ProductDto) -> Self {
        Self {
            id: product.id(),
            title: product.title().into(),
            description: product.description().into(),
            quantity: product.quantity(),
            price: product.price().to_string(),
            category: product.category().clone().into(),
        }
    }
}
