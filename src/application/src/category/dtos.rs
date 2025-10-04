use domain::Category;
use uuid::Uuid;

pub struct CategoryDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
}

impl From<Category> for CategoryDto {
    fn from(category: Category) -> Self {
        Self {
            id: category.id,
            title: category.title,
            description: category.description,
        }
    }
}
