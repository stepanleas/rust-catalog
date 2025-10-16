use crate::{CreateCategoryCommand, UpdateCategoryCommand};
use domain::Category;
use shared::domain::value_objects::CategoryId;

pub struct CategoryMapper;

impl CategoryMapper {
    pub fn map_create_category_command_to_domain_entity(
        command: &CreateCategoryCommand,
    ) -> Category {
        Category::builder()
            .id(CategoryId::new())
            .title(command.title())
            .description(command.description())
            .build()
    }

    pub fn map_update_category_command_to_domain_entity(
        command: &UpdateCategoryCommand,
    ) -> Category {
        Category::builder()
            .id(CategoryId::from_uuid(command.id()))
            .title(command.title())
            .description(command.description())
            .build()
    }
}
