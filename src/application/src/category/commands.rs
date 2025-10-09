use uuid::Uuid;

pub struct CreateCategoryCommand {
    pub title: String,
    pub description: String,
}

impl CreateCategoryCommand {
    pub fn new(title: String, description: String) -> Self {
        CreateCategoryCommand { title, description }
    }
}

pub struct UpdateCategoryCommand {
    id: Uuid,
    title: String,
    description: String,
}

impl UpdateCategoryCommand {
    pub fn new(id: Uuid, title: String, description: String) -> Self {
        UpdateCategoryCommand {
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

pub struct DeleteCategoryCommand {
    pub id: Uuid,
}

impl DeleteCategoryCommand {
    pub fn new(id: Uuid) -> Self {
        DeleteCategoryCommand { id }
    }
}
