#[cfg(test)]
mod tests {
    use crate::{PostgresCategoryRepository, configure};
    use application::CategoryRepository;
    use domain::{Category, DomainError};
    use shared::domain::value_objects::CategoryId;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    struct TestContext {
        _container: testcontainers::ContainerAsync<Postgres>,
        repository: PostgresCategoryRepository,
    }

    async fn setup_context() -> anyhow::Result<TestContext> {
        let container = Postgres::default().start().await?;
        let port = container.get_host_port_ipv4(5432).await?;
        let url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

        let db_pool = configure(url).await?;
        let repository = PostgresCategoryRepository::new(&db_pool);

        Ok(TestContext {
            _container: container,
            repository,
        })
    }

    #[tokio::test]
    async fn test_list_all() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let first = Category::new(
            CategoryId::new(),
            "category title 1".into(),
            "category description 1".into(),
        );
        let second = Category::new(
            CategoryId::new(),
            "category title 2".into(),
            "category description 2".into(),
        );
        ctx.repository.save(first)?;
        ctx.repository.save(second)?;

        let categories = ctx.repository.list_all()?;
        assert_eq!(categories.len(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_create_category() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let id = CategoryId::new();
        let category = Category::new(id, "category title".into(), "category description".into());
        ctx.repository.save(category.clone())?;

        let saved_category = ctx.repository.find_by_id(id)?;
        assert_eq!(saved_category.id(), id);
        assert_eq!(saved_category.title(), "category title");
        assert_eq!(saved_category.description(), "category description");

        Ok(())
    }

    #[tokio::test]
    async fn test_update_category() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let id = CategoryId::new();
        ctx.repository.save(Category::new(
            id,
            "category title".into(),
            "category description".into(),
        ))?;

        let updated_category = Category::builder()
            .id(id)
            .title("updated category title")
            .description("updated category description")
            .build();
        ctx.repository.save(updated_category.clone())?;

        let saved_category = ctx.repository.find_by_id(id)?;
        assert_eq!(saved_category.id(), id);
        assert_eq!(saved_category.title(), "updated category title");
        assert_eq!(saved_category.description(), "updated category description");

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_category() -> anyhow::Result<()> {
        let ctx = setup_context().await?;

        let id = CategoryId::new();
        let category = Category::new(id, "category title".into(), "category description".into());

        ctx.repository.save(category.clone())?;
        ctx.repository.delete(id)?;

        let saved_category = ctx.repository.find_by_id(id);

        assert!(matches!(
            saved_category.err().unwrap().downcast_ref::<DomainError>(),
            Some(DomainError::NotFound { .. })
        ));

        Ok(())
    }
}
