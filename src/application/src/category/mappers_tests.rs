#[cfg(test)]
mod tests {
    use crate::category::mappers::CategoryMapper;
    use crate::{CreateCategoryCommand, UpdateCategoryCommand};
    use shared::domain::value_objects::CategoryId;
    use uuid::Uuid;

    #[test]
    fn test_map_create_category_command_to_domain_entity() {
        let command =
            CreateCategoryCommand::new("category title".into(), "category description".into());
        let category = CategoryMapper::map_create_category_command_to_domain_entity(&command);

        assert_ne!(category.id().as_uuid().to_string(), Uuid::nil().to_string());
        assert_eq!(category.title(), "category title");
        assert_eq!(category.description(), "category description");
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

        assert_eq!(category.id(), category_id);
        assert_eq!(category.title(), "category title");
        assert_eq!(category.description(), "category description");
    }
}
