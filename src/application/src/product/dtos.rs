use crate::CategoryDto;
use domain::Product;
use shared::domain::value_objects::Money;
use uuid::Uuid;

pub struct ProductDto {
    id: Uuid,
    title: String,
    description: String,
    quantity: i32,
    price: Money,
    category: CategoryDto,
}

impl ProductDto {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }

    pub fn price(&self) -> &Money {
        &self.price
    }

    pub fn category(&self) -> &CategoryDto {
        &self.category
    }
}

impl From<Product> for ProductDto {
    fn from(product: Product) -> Self {
        Self {
            id: product.id(),
            title: product.title().into(),
            description: product.description().into(),
            quantity: product.quantity(),
            price: product.price().clone(),
            category: product.category().into(),
        }
    }
}
