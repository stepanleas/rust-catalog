#[cfg(test)]
mod tests {
    use crate::{PostgresCategoryRepository, PostgresProductRepository, configure};
    use application::{CategoryRepository, ProductRepository};
    use domain::{Category, DomainError, Product};
    use shared::domain::value_objects::{CategoryId, Money, ProductId};
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;
    use uuid::Uuid;

    struct TestContext {
        _container: testcontainers::ContainerAsync<Postgres>,
        category_repository: PostgresCategoryRepository,
        product_repository: PostgresProductRepository,
    }

    async fn setup_context() -> anyhow::Result<TestContext> {
        let container = Postgres::default().start().await?;
        let port = container.get_host_port_ipv4(5432).await?;
        let url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

        let db_pool = configure(url).await?;
        let category_repository = PostgresCategoryRepository::new(&db_pool);
        let product_repository = PostgresProductRepository::new(&db_pool);

        Ok(TestContext {
            _container: container,
            category_repository,
            product_repository,
        })
    }

    #[tokio::test]
    async fn test_list_all() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let first_category_id = CategoryId::new();
        let first_category = Category::new(
            first_category_id,
            "first category title".into(),
            "first category description".into(),
        );

        ctx.category_repository.save(first_category.clone())?;

        let first_product_id = ProductId::new();
        let first_product = Product::new(
            first_product_id,
            "first product title".to_string(),
            "first product description".to_string(),
            10,
            Money::from_f64(25.5)?,
            first_category,
        );

        ctx.product_repository.save(first_product)?;

        let second_category_id = CategoryId::new();
        let second_category = Category::new(
            second_category_id,
            "second category title".into(),
            "second category description".into(),
        );

        ctx.category_repository.save(second_category.clone())?;

        let second_product_id = ProductId::new();
        let second_product = Product::new(
            second_product_id,
            "second product title".to_string(),
            "second product description".to_string(),
            10,
            Money::from_f64(25.5)?,
            second_category,
        );

        ctx.product_repository.save(second_product)?;

        let products = ctx.product_repository.list_all()?;
        assert_eq!(products.len(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_create_product() -> anyhow::Result<()> {
        let ctx = setup_context().await?;
        let repository = &ctx.product_repository;

        let category_id = CategoryId::new();
        let category = Category::new(
            category_id,
            "category title".into(),
            "category description".into(),
        );

        ctx.category_repository.save(category.clone())?;

        let product_id = ProductId::new();
        let product = Product::new(
            product_id,
            "product title".to_string(),
            "product description".to_string(),
            10,
            Money::from_f64(25.5)?,
            category,
        );

        repository.save(product)?;
        let saved_product = repository.find_by_id(product_id)?;

        assert_ne!(
            saved_product.id().as_uuid().to_string(),
            Uuid::nil().to_string(),
        );
        assert_eq!(saved_product.title(), "product title");
        assert_eq!(saved_product.description(), "product description");
        assert_eq!(saved_product.quantity(), 10);
        assert_eq!(saved_product.price(), &Money::from_f64(25.5)?);

        assert_eq!(saved_product.category().id(), category_id);
        assert_eq!(saved_product.category().title(), "category title");
        assert_eq!(
            saved_product.category().description(),
            "category description",
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_update_product() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let category_id = CategoryId::new();
        let category = Category::new(
            category_id,
            "category title".into(),
            "category description".into(),
        );

        ctx.category_repository.save(category.clone())?;

        let product_id = ProductId::new();
        let product = Product::new(
            product_id,
            "product title".to_string(),
            "product description".to_string(),
            10,
            Money::from_f64(25.5)?,
            category,
        );

        ctx.product_repository.save(product)?;

        let updated_product = Product::builder()
            .id(product_id)
            .title("updated product title")
            .description("updated product description")
            .quantity(20)
            .price(Money::from_f64(30.0)?)
            .category(ctx.category_repository.find_by_id(category_id)?)
            .build();
        ctx.product_repository.save(updated_product.clone())?;

        assert_ne!(
            updated_product.id().as_uuid().to_string(),
            Uuid::nil().to_string(),
        );
        assert_eq!(updated_product.title(), "updated product title");
        assert_eq!(updated_product.description(), "updated product description");
        assert_eq!(updated_product.quantity(), 20);
        assert_eq!(updated_product.price(), &Money::from_f64(30.0)?);

        assert_eq!(updated_product.category().id(), category_id);
        assert_eq!(updated_product.category().title(), "category title");
        assert_eq!(
            updated_product.category().description(),
            "category description",
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_product() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let category_id = CategoryId::new();
        let category = Category::new(
            category_id,
            "category title".into(),
            "category description".into(),
        );

        ctx.category_repository.save(category.clone())?;

        let product_id = ProductId::new();
        let product = Product::new(
            product_id,
            "product title".to_string(),
            "product description".to_string(),
            10,
            Money::from_f64(25.5)?,
            category,
        );

        ctx.product_repository.save(product)?;
        ctx.product_repository.delete(product_id)?;

        let saved_product = ctx.product_repository.find_by_id(product_id);

        assert!(matches!(
            saved_product.err().unwrap().downcast_ref::<DomainError>(),
            Some(DomainError::NotFound { .. })
        ));

        Ok(())
    }
}
