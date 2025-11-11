#[cfg(test)]
mod tests {
    use crate::category::commands::{
        CreateCategoryCommand, DeleteCategoryCommand, UpdateCategoryCommand,
    };
    use crate::category::handlers::{
        CreateCategoryCommandHandler, DeleteCategoryCommandHandler, FindCategoryQueryHandler,
        ListAllCategoryQueryHandler, UpdateCategoryCommandHandler,
    };
    use crate::category::queries::FindCategoryQuery;
    use crate::category::repositories::MockCategoryRepository;
    use domain::entities::Category;
    use mockall::predicate;
    use shared::domain::value_objects::CategoryId;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_list_all_category_query_handler_execute() -> anyhow::Result<()> {
        let mut mock_repository = MockCategoryRepository::new();

        let expected_categories = vec![
            Category::new(
                CategoryId::new(),
                "Books".to_string(),
                "A category for all book products".to_string(),
            ),
            Category::new(
                CategoryId::new(),
                "Electronics".to_string(),
                "A category for all electronic products".to_string(),
            ),
        ];

        mock_repository
            .expect_list_all()
            .times(1)
            .returning(move || Ok(expected_categories.clone()));

        let handler = ListAllCategoryQueryHandler::new(Arc::new(mock_repository));
        let category_dtos = handler.execute().await?;

        assert_eq!(category_dtos.len(), 2);
        assert_eq!(category_dtos[0].title(), "Books");
        assert_eq!(
            category_dtos[0].description(),
            "A category for all book products"
        );
        assert_eq!(category_dtos[1].title(), "Electronics");
        assert_eq!(
            category_dtos[1].description(),
            "A category for all electronic products"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_find_category_query_handler_execute() -> anyhow::Result<()> {
        let mut mock_repository = MockCategoryRepository::new();

        let category_id = CategoryId::from_uuid(Uuid::new_v4());

        mock_repository
            .expect_find_by_id()
            .with(predicate::eq(category_id))
            .times(1)
            .returning(move |_| {
                Ok(Category::new(
                    category_id,
                    "Books".to_string(),
                    "A category for all book products".to_string(),
                ))
            });

        let handler = FindCategoryQueryHandler::new(Arc::new(mock_repository));

        let category_dto = handler
            .execute(FindCategoryQuery::new(Option::from(*category_id.as_uuid())))
            .await?;

        assert_eq!(category_dto.title(), "Books");
        assert_eq!(
            category_dto.description(),
            "A category for all book products",
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_create_category_command_handler_execute() -> anyhow::Result<()> {
        let mut mock_repository = MockCategoryRepository::new();

        mock_repository
            .expect_save()
            .times(1)
            .withf(move |category: &Category| {
                category.title() == "Books"
                    && category.description() == "A category for all book products"
            })
            .returning(|category| Ok(category.clone()));

        let handler = CreateCategoryCommandHandler::new(Arc::new(mock_repository));

        let command = CreateCategoryCommand::new(
            "Books".to_string(),
            "A category for all book products".to_string(),
        );
        let category_dto = handler.execute(command).await?;

        assert_eq!(category_dto.title(), "Books");
        assert_eq!(
            category_dto.description(),
            "A category for all book products"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_update_category_command_execute() -> anyhow::Result<()> {
        let mut mock_repository = MockCategoryRepository::new();

        let category_id = Uuid::new_v4();

        mock_repository
            .expect_save()
            .times(1)
            .withf(move |category: &Category| {
                category.id() == CategoryId::from_uuid(category_id)
                    && category.title() == "Books"
                    && category.description() == "A category for all book products"
            })
            .returning(|category| Ok(category.clone()));

        let handler = UpdateCategoryCommandHandler::new(Arc::new(mock_repository));

        let command = UpdateCategoryCommand::new(
            category_id,
            "Books".to_string(),
            "A category for all book products".to_string(),
        );
        let category_dto = handler.execute(command).await?;

        assert_eq!(category_dto.id(), category_id);
        assert_eq!(category_dto.title(), "Books");
        assert_eq!(
            category_dto.description(),
            "A category for all book products"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_category_command_handler_execute() -> anyhow::Result<()> {
        let mut mock_repository = MockCategoryRepository::new();

        let command = DeleteCategoryCommand::new(Uuid::new_v4());

        mock_repository
            .expect_delete()
            .times(1)
            .returning(|_| Ok(()));

        let handler = DeleteCategoryCommandHandler::new(Arc::new(mock_repository));

        handler.execute(command).await?;

        Ok(())
    }
}
