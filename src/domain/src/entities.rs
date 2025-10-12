use crate::builders::{CategoryBuilder, ProductBuilder};
use shared::domain::value_objects::Money;
use uuid::Uuid;

#[derive(Default)]
pub struct Category {
    id: Uuid,
    title: String,
    description: String,
}

impl Category {
    pub fn builder() -> CategoryBuilder {
        CategoryBuilder::default()
    }

    pub fn new(id: Uuid, title: String, description: String) -> Self {
        Category {
            id,
            title,
            description,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

pub struct Product {
    id: Uuid,
    title: String,
    description: String,
    quantity: i32,
    price: Money,
    category: Category,
}

impl Product {
    pub fn builder() -> ProductBuilder {
        ProductBuilder::default()
    }

    pub fn new(
        id: Uuid,
        title: String,
        description: String,
        quantity: i32,
        price: Money,
        category: Category,
    ) -> Self {
        Product {
            id,
            title,
            description,
            quantity,
            price,
            category,
        }
    }

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

    pub fn category(&self) -> &Category {
        &self.category
    }
}
