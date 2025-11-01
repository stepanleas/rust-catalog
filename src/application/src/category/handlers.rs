use crate::DeleteCategoryCommand;
use crate::category::commands::{CreateCategoryCommand, UpdateCategoryCommand};
use crate::category::dtos::CategoryDto;
use crate::category::mappers::CategoryMapper;
use crate::category::queries::FindCategoryQuery;
use crate::category::repositories::CategoryRepository;
use shared::domain::value_objects::CategoryId;
use std::sync::Arc;

pub struct FindCategoryQueryHandler {
    repository: Arc<dyn CategoryRepository>,
}

impl FindCategoryQueryHandler {
    pub fn new(repository: Arc<dyn CategoryRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, query: FindCategoryQuery) -> anyhow::Result<CategoryDto> {
        self.repository
            .find_by_id(CategoryId::from_uuid(query.id().unwrap()))
            .map(CategoryDto::from)
    }
}

pub struct ListAllCategoryQueryHandler {
    repository: Arc<dyn CategoryRepository>,
}

impl ListAllCategoryQueryHandler {
    pub fn new(repository: Arc<dyn CategoryRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> anyhow::Result<Vec<CategoryDto>> {
        self.repository
            .list_all()
            .map(|items| items.into_iter().map(CategoryDto::from).collect())
    }
}

pub struct CreateCategoryCommandHandler {
    repository: Arc<dyn CategoryRepository>,
}

impl CreateCategoryCommandHandler {
    pub fn new(repository: Arc<dyn CategoryRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreateCategoryCommand) -> anyhow::Result<CategoryDto> {
        let category = CategoryMapper::map_create_category_command_to_domain_entity(&command);

        self.repository.save(category).map(CategoryDto::from)
    }
}

pub struct UpdateCategoryCommandHandler {
    repository: Arc<dyn CategoryRepository>,
}

impl UpdateCategoryCommandHandler {
    pub fn new(repository: Arc<dyn CategoryRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: UpdateCategoryCommand) -> anyhow::Result<CategoryDto> {
        let category = CategoryMapper::map_update_category_command_to_domain_entity(&command);

        self.repository.save(category).map(CategoryDto::from)
    }
}

pub struct DeleteCategoryCommandHandler {
    repository: Arc<dyn CategoryRepository>,
}

impl DeleteCategoryCommandHandler {
    pub fn new(repository: Arc<dyn CategoryRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: DeleteCategoryCommand) -> anyhow::Result<()> {
        self.repository.delete(CategoryId::from_uuid(command.id()))
    }
}
