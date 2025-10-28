use crate::Product;
use chrono::{DateTime, Utc};

pub struct ProductCreatedEvent {
    product: Product,
    created_at: DateTime<Utc>,
}

impl ProductCreatedEvent {
    pub fn new(product: Product) -> Self {
        Self {
            product,
            created_at: Utc::now(),
        }
    }

    pub fn product(&self) -> &Product {
        &self.product
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

pub struct ProductUpdatedEvent {
    product: Product,
    created_at: DateTime<Utc>,
}

impl ProductUpdatedEvent {
    pub fn new(product: Product) -> Self {
        Self {
            product,
            created_at: Utc::now(),
        }
    }

    pub fn product(&self) -> &Product {
        &self.product
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
