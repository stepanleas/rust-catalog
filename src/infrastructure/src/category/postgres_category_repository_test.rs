#[cfg(test)]
mod tests {
    use crate::{PostgresCategoryRepository, configure};
    use application::CategoryRepository;
    use domain::Category;
    use shared::domain::value_objects::CategoryId;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    struct TestContext {
        _container: testcontainers::ContainerAsync<Postgres>,
        repository: PostgresCategoryRepository,
    }

    async fn setup_repository() -> anyhow::Result<TestContext> {
        let container = Postgres::default().start().await?;
        let port = container.get_host_port_ipv4(5432).await?;
        let url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let db_pool = configure(url).await?;
        let repository = PostgresCategoryRepository::new(&db_pool);

        Ok(TestContext {
            _container: container,
            repository,
        })
    }

    #[tokio::test]
    async fn test_list_all() -> anyhow::Result<()> {
        let ctx = setup_repository().await?;

        let first = Category::new(CategoryId::new(), "Title 1".into(), "Desc 1".into());
        let second = Category::new(CategoryId::new(), "Title 2".into(), "Desc 2".into());
        ctx.repository.save(first)?;
        ctx.repository.save(second)?;

        let categories = ctx.repository.list_all()?;
        assert_eq!(categories.len(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_create_category() -> anyhow::Result<()> {
        let ctx = setup_repository().await?;

        let id = CategoryId::new();
        let category = Category::new(id, "Category title".into(), "Category description".into());
        ctx.repository.save(category.clone())?;

        let saved_category = ctx.repository.find_by_id(id)?;
        assert_eq!(saved_category.id(), id);
        assert_eq!(saved_category.title(), "Category title");
        assert_eq!(saved_category.description(), "Category description");

        Ok(())
    }
}
