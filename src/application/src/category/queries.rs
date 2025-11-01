use uuid::Uuid;

pub struct FindCategoryQuery {
    id: Option<Uuid>,
}

impl FindCategoryQuery {
    pub fn new(id: Option<Uuid>) -> Self {
        Self { id }
    }

    pub fn id(&self) -> Option<Uuid> {
        self.id
    }
}
