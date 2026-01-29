#[cfg(test)]
mod tests {
    use crate::product::commands::{CreateProductCommand, UpdateProductCommand};
    use crate::product::mappers::ProductMapper;
    use domain::entities::Category;
    use shared::domain::value_objects::{CategoryId, ProductId};
    use uuid::Uuid;

    #[test]
    fn test_create_product_command_to_domain_entity() -> anyhow::Result<()> {
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
            "25.5".into(),
            category.id().into(),
        );
        let product =
            ProductMapper::map_create_product_command_to_domain_entity(&command, category)?;

        assert_ne!(Uuid::nil().to_string(), product.id().as_uuid().to_string());
        assert_eq!("product title", product.title());
        assert_eq!("product description", product.description());
        assert_eq!(10, product.quantity());
        assert_eq!("$25.50", product.price().to_string());

        assert_eq!(category_id, product.category().id());
        assert_eq!("category title", product.category().title());
        assert_eq!("category description", product.category().description());

        Ok(())
    }

    #[test]
    fn test_update_product_command_to_domain_entity() -> anyhow::Result<()> {
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
            "25.5".into(),
            category.id().into(),
        );
        let product =
            ProductMapper::map_update_product_command_to_domain_entity(&command, category)?;

        assert_eq!(product_id, product.id());
        assert_eq!("product title", product.title());
        assert_eq!("product description", product.description());
        assert_eq!(10, product.quantity());
        assert_eq!("$25.50", product.price().to_string());

        assert_eq!(category_id, product.category().id());
        assert_eq!("category title", product.category().title());
        assert_eq!("category description", product.category().description());

        Ok(())
    }
}
