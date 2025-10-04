use uuid::Uuid;

pub struct FindCategoryQuery {
    pub id: Option<Uuid>,
}

impl FindCategoryQuery {
    pub fn new(id: Option<Uuid>) -> Self {
        Self { id }
    }
}
