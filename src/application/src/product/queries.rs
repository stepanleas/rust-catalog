use uuid::Uuid;

pub struct FindProductQuery {
    pub id: Option<Uuid>,
}

impl FindProductQuery {
    pub fn new(id: Option<Uuid>) -> Self {
        Self { id }
    }
}
