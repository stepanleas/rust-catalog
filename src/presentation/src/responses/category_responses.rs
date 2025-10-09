use application::CategoryDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[readonly::make]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CategoryResponse {
    id: Uuid,
    title: String,
    description: String,
}

impl From<CategoryDto> for CategoryResponse {
    fn from(category: CategoryDto) -> Self {
        Self {
            id: category.id(),
            title: category.title().into(),
            description: category.description().into(),
        }
    }
}
