#[cfg(test)]
mod tests {
    use crate::category::commands::{CreateCategoryCommand, UpdateCategoryCommand};
    use crate::category::mappers::CategoryMapper;
    use shared::domain::value_objects::CategoryId;
    use uuid::Uuid;

    #[test]
    fn test_map_create_category_command_to_domain_entity() {
        let command =
            CreateCategoryCommand::new("category title".into(), "category description".into());
        let category = CategoryMapper::map_create_category_command_to_domain_entity(&command);

        assert_ne!(Uuid::nil().to_string(), category.id().as_uuid().to_string());
        assert_eq!("category title", category.title());
        assert_eq!("category description", category.description());
    }

    #[test]
    fn test_map_update_category_command_to_domain_entity() {
        let category_id = CategoryId::new();
        let command = UpdateCategoryCommand::new(
            category_id.into(),
            "category title".into(),
            "category description".into(),
        );
        let category = CategoryMapper::map_update_category_command_to_domain_entity(&command);

        assert_eq!(category_id, category.id());
        assert_eq!("category title", category.title());
        assert_eq!("category description", category.description());
    }
}
