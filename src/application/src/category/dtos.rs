use domain::Category;
use uuid::Uuid;

#[derive(Clone)]
pub struct CategoryDto {
    id: Uuid,
    title: String,
    description: String,
}

impl CategoryDto {
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

impl From<&Category> for CategoryDto {
    fn from(category: &Category) -> Self {
        Self {
            id: category.id(),
            title: category.title().into(),
            description: category.description().into(),
        }
    }
}

impl From<Category> for CategoryDto {
    fn from(category: Category) -> Self {
        Self {
            id: category.id(),
            title: category.title().into(),
            description: category.description().into(),
        }
    }
}
