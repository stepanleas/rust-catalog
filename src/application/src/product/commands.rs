use uuid::Uuid;

pub struct CreateProductCommand {
    title: String,
    description: String,
    quantity: i32,
    price: f64,
    category_id: Uuid,
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

impl CreateProductCommand {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn category_id(&self) -> Uuid {
        self.category_id
    }
}

pub struct UpdateProductCommand {
    id: Uuid,
    title: String,
    description: String,
    quantity: i32,
    price: f64,
    category_id: Uuid,
}

impl UpdateProductCommand {
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

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn category_id(&self) -> Uuid {
        self.category_id
    }
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
