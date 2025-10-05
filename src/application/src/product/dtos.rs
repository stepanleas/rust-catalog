use crate::CategoryDto;
use domain::Product;
use shared::domain::value_objects::Money;
use uuid::Uuid;

pub struct ProductDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub quantity: i32,
    pub price: Money,
    pub category: CategoryDto,
}

impl From<Product> for ProductDto {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            title: product.title,
            description: product.description,
            quantity: product.quantity,
            price: product.price,
            category: CategoryDto::from(product.category),
        }
    }
}
