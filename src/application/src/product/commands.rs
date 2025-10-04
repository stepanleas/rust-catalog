use uuid::Uuid;

pub struct CreateProductCommand {
    pub title: String,
    pub description: String,
    pub quantity: i32,
    pub price: f64,
    pub category_id: Uuid,
}

impl CreateProductCommand {
    pub fn new(
        title: String,
        description: String,
        quantity: i32,
        price: f64,
        category_id: Uuid,
    ) -> Self {
        CreateProductCommand {
            title,
            description,
            quantity,
            price,
            category_id,
        }
    }
}

pub struct UpdateProductCommand {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub quantity: i32,
    pub price: f64,
    pub category_id: Uuid,
}

impl UpdateProductCommand {
    pub fn new(
        id: Uuid,
        title: String,
        description: String,
        quantity: i32,
        price: f64,
        category_id: Uuid,
    ) -> Self {
        UpdateProductCommand {
            id,
            title,
            description,
            quantity,
            price,
            category_id,
        }
    }
}

pub struct DeleteProductCommand {
    pub id: Uuid,
}

impl DeleteProductCommand {
    pub fn new(id: Uuid) -> Self {
        DeleteProductCommand { id }
    }
}
