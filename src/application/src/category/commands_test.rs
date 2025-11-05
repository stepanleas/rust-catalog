#[cfg(test)]
mod tests {
    use crate::category::commands::{
        CreateCategoryCommand, DeleteCategoryCommand, UpdateCategoryCommand,
    };

    #[test]
    fn test_create_category_command() {
        let command = CreateCategoryCommand::new(
            "Books".to_string(),
            "A category for all book products".to_string(),
        );

        assert_eq!(command.title(), "Books");
        assert_eq!(command.description(), "A category for all book products");
    }

    #[test]
    fn test_update_category_command() {
        let category_id = uuid::Uuid::new_v4();
        let command = UpdateCategoryCommand::new(
            category_id,
            "Books".to_string(),
            "A category for all book products".to_string(),
        );

        assert_eq!(command.id(), category_id);
        assert_eq!(command.title(), "Books");
        assert_eq!(command.description(), "A category for all book products");
    }

    #[test]
    fn test_delete_category_command() {
        let category_id = uuid::Uuid::new_v4();
        let command = DeleteCategoryCommand::new(category_id);

        assert_eq!(command.id(), category_id);
    }
}
