#[cfg(test)]
mod tests {
    use crate::product::commands::{
        CreateProductCommand, DeleteProductCommand, UpdateProductCommand,
    };
    use uuid::Uuid;

    #[test]
    fn test_create_product_command() {
        let category_id = Uuid::new_v4();
        let command = CreateProductCommand::new(
            "Test Product".to_string(),
            "A product for testing".to_string(),
            10,
            99.99,
            category_id,
        );

        assert_eq!(command.title(), "Test Product");
        assert_eq!(command.description(), "A product for testing");
        assert_eq!(command.quantity(), 10);
        assert_eq!(command.price(), 99.99);
        assert_eq!(command.category_id(), category_id);
    }

    #[test]
    fn test_update_product_command() {
        let product_id = Uuid::new_v4();
        let category_id = Uuid::new_v4();
        let command = UpdateProductCommand::new(
            product_id,
            "Test Product".to_string(),
            "A product for testing".to_string(),
            10,
            99.99,
            category_id,
        );

        assert_eq!(command.id(), product_id);
        assert_eq!(command.title(), "Test Product");
        assert_eq!(command.description(), "A product for testing");
        assert_eq!(command.quantity(), 10);
        assert_eq!(command.price(), 99.99);
        assert_eq!(command.category_id(), category_id);
    }

    #[test]
    fn test_delete_product_command() {
        let product_id = Uuid::new_v4();
        let command = DeleteProductCommand::new(product_id);

        assert_eq!(command.id(), product_id);
    }
}
