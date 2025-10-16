#[cfg(test)]
mod tests {
    use crate::product::mappers::ProductMapper;
    use crate::{CreateProductCommand, UpdateProductCommand};
    use domain::Category;
    use shared::domain::value_objects::{CategoryId, Money, ProductId};
    use uuid::Uuid;

    #[test]
    fn test_create_product_command_to_domain_entity() {
        let category_id = CategoryId::new();
        let category = Category::new(
            category_id,
            "category title".into(),
            "category description".into(),
        );

        let command = CreateProductCommand::new(
            "product title".to_string(),
            "product description".to_string(),
            10,
            25.5,
            category.id().into(),
        );
        let product =
            ProductMapper::map_create_product_command_to_domain_entity(&command, category).unwrap();

        assert_ne!(product.id().as_uuid().to_string(), Uuid::nil().to_string());
        assert_eq!(product.title(), "product title");
        assert_eq!(product.description(), "product description");
        assert_eq!(product.quantity(), 10);
        assert_eq!(product.price(), &Money::from_f64(25.5).unwrap());

        assert_eq!(product.category().id(), category_id);
        assert_eq!(product.category().title(), "category title");
        assert_eq!(product.category().description(), "category description");
    }

    #[test]
    fn test_update_product_command_to_domain_entity() {
        let category_id = CategoryId::new();
        let category = Category::new(
            category_id,
            "category title".into(),
            "category description".into(),
        );

        let product_id = ProductId::new();
        let command = UpdateProductCommand::new(
            product_id.into(),
            "product title".to_string(),
            "product description".to_string(),
            10,
            25.5,
            category.id().into(),
        );
        let product =
            ProductMapper::map_update_product_command_to_domain_entity(&command, category).unwrap();

        assert_eq!(product.id(), product_id);
        assert_eq!(product.title(), "product title");
        assert_eq!(product.description(), "product description");
        assert_eq!(product.quantity(), 10);
        assert_eq!(product.price(), &Money::from_f64(25.5).unwrap());

        assert_eq!(product.category().id(), category_id);
        assert_eq!(product.category().title(), "category title");
        assert_eq!(product.category().description(), "category description");
    }
}
