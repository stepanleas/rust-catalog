use crate::{Category, Product};
use shared::domain::value_objects::Money;
use uuid::Uuid;

#[derive(Default)]
pub struct CategoryBuilder {
    id: Uuid,
    title: String,
    description: String,
}

impl CategoryBuilder {
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn build(self) -> Category {
        Category::new(self.id, self.title, self.description)
    }
}

#[derive(Default)]
pub struct ProductBuilder {
    id: Uuid,
    title: String,
    description: String,
    quantity: i32,
    price: Money,
    category: Category,
}

impl ProductBuilder {
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn quantity(mut self, quantity: i32) -> Self {
        self.quantity = quantity;
        self
    }

    pub fn price(mut self, price: Money) -> Self {
        self.price = price;
        self
    }

    pub fn category(mut self, category: Category) -> Self {
        self.category = category;
        self
    }

    pub fn build(self) -> Product {
        Product::new(
            self.id,
            self.title,
            self.description,
            self.quantity,
            self.price,
            self.category,
        )
    }
}
