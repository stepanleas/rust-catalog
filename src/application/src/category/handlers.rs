use crate::DeleteCategoryCommand;
use crate::category::commands::{CreateCategoryCommand, UpdateCategoryCommand};
use crate::category::dtos::CategoryDto;
use crate::category::queries::FindCategoryQuery;
use crate::category::repositories::CategoryRepository;
use domain::Category;
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
            .find_by_id(query.id.unwrap())
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
        self.repository
            .save(Category::new(command.title, command.description))
            .map(CategoryDto::from)
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
        self.repository
            .save(Category::new_with_id(
                command.id(),
                command.title().into(),
                command.description().into(),
            ))
            .map(CategoryDto::from)
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
        self.repository.delete(command.id)
    }
}
