use crate::value_objects::Money;
use uuid::Uuid;

pub struct Category {
    pub id: Uuid,
    pub title: String,
    pub description: String,
}

impl Category {
    pub fn new(title: String, description: String) -> Self {
        Category {
            id: Uuid::new_v4(),
            title,
            description,
        }
    }
}

pub struct Product {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub quantity: i32,
    pub price: Money,
    pub category: Category,
}

impl Product {
    pub fn new(
        title: String,
        description: String,
        quantity: i32,
        price: Money,
        category: Category,
    ) -> Self {
        Product {
            id: Uuid::new_v4(),
            title,
            description,
            quantity,
            price,
            category,
        }
    }
}
