use crate::builders::{CategoryBuilder, ProductBuilder};
use shared::domain::value_objects::{CategoryId, Money, ProductId};

#[derive(Default, Clone)]
pub struct Category {
    id: CategoryId,
    title: String,
    description: String,
}

impl Category {
    pub fn builder() -> CategoryBuilder {
        CategoryBuilder::default()
    }

    pub fn new(id: CategoryId, title: String, description: String) -> Self {
        Category {
            id,
            title,
            description,
        }
    }

    pub fn id(&self) -> CategoryId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Clone)]
pub struct Product {
    id: ProductId,
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
        id: ProductId,
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

    pub fn id(&self) -> ProductId {
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
