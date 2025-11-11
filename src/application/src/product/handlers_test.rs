#[cfg(test)]
mod tests {
    use crate::category::repositories::MockCategoryRepository;
    use crate::ports::output::publishers::MockProductMessagePublisher;
    use crate::product::commands::{
        CreateProductCommand, DeleteProductCommand, UpdateProductCommand,
    };
    use crate::product::handlers::{
        CreateProductCommandHandler, DeleteProductCommandHandler, FindProductQueryHandler,
        ListAllProductQueryHandler, UpdateProductCommandHandler,
    };
    use crate::product::queries::FindProductQuery;
    use crate::product::repositories::MockProductRepository;
    use domain::entities::{Category, Product};
    use mockall::predicate;
    use shared::domain::value_objects::{CategoryId, Money, ProductId};
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_list_all_product_query_handler_execute() -> anyhow::Result<()> {
        let mut mock_repository = MockProductRepository::new();

        let first_category = Category::new(
            CategoryId::new(),
            "Electronics".to_string(),
            "Electronic devices and gadgets".to_string(),
        );
        let second_category = Category::new(
            CategoryId::new(),
            "Home Appliances".to_string(),
            "Appliances for home use".to_string(),
        );

        let expected_products = vec![
            Product::new(
                ProductId::new(),
                "Laptop".to_string(),
                "A high-performance laptop".to_string(),
                15,
                Money::from_f64(30.5)?,
                first_category,
            ),
            Product::new(
                ProductId::new(),
                "Smartphone".to_string(),
                "A latest model smartphone".to_string(),
                7,
                Money::from_f64(13.75)?,
                second_category,
            ),
        ];

        mock_repository
            .expect_list_all()
            .times(1)
            .returning(move || Ok(expected_products.clone()));

        let handler = ListAllProductQueryHandler::new(Arc::new(mock_repository));
        let product_dtos = handler.execute().await?;

        assert_eq!(product_dtos.len(), 2);

        assert_eq!(product_dtos[0].title(), "Laptop");
        assert_eq!(product_dtos[0].description(), "A high-performance laptop");
        assert_eq!(product_dtos[0].quantity(), 15);
        assert_eq!(product_dtos[0].price(), &Money::from_f64(30.5)?);
        assert_eq!(product_dtos[0].category().title(), "Electronics");
        assert_eq!(
            product_dtos[0].category().description(),
            "Electronic devices and gadgets",
        );

        assert_eq!(product_dtos[1].title(), "Smartphone");
        assert_eq!(product_dtos[1].description(), "A latest model smartphone");
        assert_eq!(product_dtos[1].quantity(), 7);
        assert_eq!(product_dtos[1].price(), &Money::from_f64(13.75)?);
        assert_eq!(product_dtos[1].category().title(), "Home Appliances");
        assert_eq!(
            product_dtos[1].category().description(),
            "Appliances for home use",
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_find_product_query_handler_execute() -> anyhow::Result<()> {
        let mut mock_repository = MockProductRepository::new();

        let category = Category::new(
            CategoryId::new(),
            "Electronics".to_string(),
            "Electronic devices and gadgets".to_string(),
        );

        let product_id = ProductId::new();
        let expected_product = Product::new(
            product_id,
            "Laptop".to_string(),
            "A high-performance laptop".to_string(),
            15,
            Money::from_f64(30.5)?,
            category,
        );

        mock_repository
            .expect_find_by_id()
            .with(predicate::eq(product_id))
            .times(1)
            .returning(move |_| Ok(expected_product.clone()));

        let handler = FindProductQueryHandler::new(Arc::new(mock_repository));
        let product_dto = handler
            .execute(FindProductQuery::new(Option::from(*product_id.as_uuid())))
            .await?;

        assert_eq!(product_dto.title(), "Laptop");
        assert_eq!(product_dto.description(), "A high-performance laptop");
        assert_eq!(product_dto.quantity(), 15);
        assert_eq!(product_dto.price(), &Money::from_f64(30.5)?);
        assert_eq!(product_dto.category().title(), "Electronics");
        assert_eq!(
            product_dto.category().description(),
            "Electronic devices and gadgets",
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_create_product_command_handler_execute() -> anyhow::Result<()> {
        let mut mock_category_repository = MockCategoryRepository::new();
        let mut mock_product_repository = MockProductRepository::new();
        let mut mock_message_publisher = MockProductMessagePublisher::new();

        let category_id = Uuid::new_v4();

        mock_category_repository
            .expect_find_by_id()
            .with(predicate::eq(CategoryId::from_uuid(category_id)))
            .times(1)
            .returning(move |_| {
                Ok(Category::new(
                    CategoryId::new(),
                    "Electronics".to_string(),
                    "Electronic devices and gadgets".to_string(),
                ))
            });

        mock_product_repository
            .expect_save()
            .times(1)
            .withf(move |product| {
                product.title() == "Laptop"
                    && product.description() == "A high-performance laptop"
                    && product.quantity() == 15
                    && product.price() == &Money::from_f64(30.5).unwrap()
                    && product.category().title() == "Electronics"
                    && product.category().description() == "Electronic devices and gadgets"
            })
            .returning(Ok);

        mock_message_publisher
            .expect_publish_created()
            .times(1)
            .withf(|event| {
                event.product().title() == "Laptop"
                    && event.product().description() == "A high-performance laptop"
                    && event.product().quantity() == 15
                    && event.product().price() == &Money::from_f64(30.5).unwrap()
                    && event.product().category().title() == "Electronics"
                    && event.product().category().description() == "Electronic devices and gadgets"
            })
            .returning(|_| Ok(()));

        let command = CreateProductCommand::new(
            "Laptop".to_string(),
            "A high-performance laptop".to_string(),
            15,
            30.5,
            category_id,
        );

        let handler = CreateProductCommandHandler::new(
            Arc::new(mock_product_repository),
            Arc::new(mock_category_repository),
            Arc::new(mock_message_publisher),
        );

        let product_dto = handler.execute(command).await?;
        assert_eq!(product_dto.title(), "Laptop");
        assert_eq!(product_dto.description(), "A high-performance laptop");
        assert_eq!(product_dto.quantity(), 15);
        assert_eq!(product_dto.price(), &Money::from_f64(30.5)?);

        assert_eq!(product_dto.category().title(), "Electronics");
        assert_eq!(
            product_dto.category().description(),
            "Electronic devices and gadgets",
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_update_product_command_handler_execute() -> anyhow::Result<()> {
        let mut mock_category_repository = MockCategoryRepository::new();
        let mut mock_product_repository = MockProductRepository::new();
        let mut mock_message_publisher = MockProductMessagePublisher::new();

        let product_id = Uuid::new_v4();
        let category_id = Uuid::new_v4();

        mock_category_repository
            .expect_find_by_id()
            .with(predicate::eq(CategoryId::from_uuid(category_id)))
            .times(1)
            .returning(move |_| {
                Ok(Category::new(
                    CategoryId::new(),
                    "Electronics".to_string(),
                    "Electronic devices and gadgets".to_string(),
                ))
            });

        mock_product_repository
            .expect_save()
            .times(1)
            .withf(move |product| {
                product.id() == ProductId::from_uuid(product_id)
                    && product.title() == "Laptop"
                    && product.description() == "A high-performance laptop"
                    && product.quantity() == 15
                    && product.price() == &Money::from_f64(30.5).unwrap()
                    && product.category().title() == "Electronics"
                    && product.category().description() == "Electronic devices and gadgets"
            })
            .returning(Ok);

        mock_message_publisher
            .expect_publish_updated()
            .times(1)
            .withf(move |event| {
                event.product().id() == ProductId::from_uuid(product_id)
                    && event.product().title() == "Laptop"
                    && event.product().description() == "A high-performance laptop"
                    && event.product().quantity() == 15
                    && event.product().price() == &Money::from_f64(30.5).unwrap()
                    && event.product().category().title() == "Electronics"
                    && event.product().category().description() == "Electronic devices and gadgets"
            })
            .returning(|_| Ok(()));

        let command = UpdateProductCommand::new(
            product_id,
            "Laptop".to_string(),
            "A high-performance laptop".to_string(),
            15,
            30.5,
            category_id,
        );

        let handler = UpdateProductCommandHandler::new(
            Arc::new(mock_product_repository),
            Arc::new(mock_category_repository),
            Arc::new(mock_message_publisher),
        );

        let product_dto = handler.execute(command).await?;
        assert_eq!(product_dto.title(), "Laptop");
        assert_eq!(product_dto.description(), "A high-performance laptop");
        assert_eq!(product_dto.quantity(), 15);
        assert_eq!(product_dto.price(), &Money::from_f64(30.5)?);

        assert_eq!(product_dto.category().title(), "Electronics");
        assert_eq!(
            product_dto.category().description(),
            "Electronic devices and gadgets",
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_product_command_handler_execute() -> anyhow::Result<()> {
        let mut mock_repository = MockProductRepository::new();
        let mut mock_message_publisher = MockProductMessagePublisher::new();

        let product_id = Uuid::new_v4();

        let command = DeleteProductCommand::new(product_id);

        mock_repository
            .expect_delete()
            .times(1)
            .returning(|_| Ok(()));
        mock_message_publisher
            .expect_publish_deleted()
            .times(1)
            .withf(move |event| event.product_id() == ProductId::from_uuid(product_id))
            .returning(|_| Ok(()));

        let handler = DeleteProductCommandHandler::new(
            Arc::new(mock_repository),
            Arc::new(mock_message_publisher),
        );

        handler.execute(command).await?;

        Ok(())
    }
}
