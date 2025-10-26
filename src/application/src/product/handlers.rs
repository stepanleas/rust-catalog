use crate::ports::output::ProductMessagePublisher;
use crate::product::commands::{CreateProductCommand, DeleteProductCommand, UpdateProductCommand};
use crate::product::dtos::ProductDto;
use crate::product::mappers::ProductMapper;
use crate::product::repositories::ProductRepository;
use crate::{CategoryRepository, FindProductQuery};
use domain::ProductCreatedEvent;
use std::sync::Arc;

pub struct FindProductQueryHandler {
    repository: Arc<dyn ProductRepository>,
}

impl FindProductQueryHandler {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, query: FindProductQuery) -> anyhow::Result<ProductDto> {
        self.repository
            .find_by_id(query.id.unwrap())
            .map(ProductDto::from)
    }
}

pub struct ListAllProductQueryHandler {
    repository: Arc<dyn ProductRepository>,
}

impl ListAllProductQueryHandler {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> anyhow::Result<Vec<ProductDto>> {
        self.repository
            .list_all()
            .map(|items| items.into_iter().map(ProductDto::from).collect())
    }
}

pub struct CreateProductCommandHandler {
    category_repository: Arc<dyn CategoryRepository>,
    product_repository: Arc<dyn ProductRepository>,
    message_publisher: Arc<dyn ProductMessagePublisher>,
}

impl CreateProductCommandHandler {
    pub fn new(
        product_repository: Arc<dyn ProductRepository>,
        category_repository: Arc<dyn CategoryRepository>,
        message_publisher: Arc<dyn ProductMessagePublisher>,
    ) -> Self {
        Self {
            product_repository,
            category_repository,
            message_publisher,
        }
    }

    pub async fn execute(&self, command: CreateProductCommand) -> anyhow::Result<ProductDto> {
        let category = self.category_repository.find_by_id(command.category_id())?;
        let product =
            ProductMapper::map_create_product_command_to_domain_entity(&command, category)?;

        let product = self.product_repository.save(product)?;

        self.message_publisher
            .publish_created(ProductCreatedEvent::new(product.clone()))?;

        Ok(ProductDto::from(product))
    }
}

pub struct UpdateProductCommandHandler {
    category_repository: Arc<dyn CategoryRepository>,
    product_repository: Arc<dyn ProductRepository>,
}

impl UpdateProductCommandHandler {
    pub fn new(
        product_repository: Arc<dyn ProductRepository>,
        category_repository: Arc<dyn CategoryRepository>,
    ) -> Self {
        Self {
            product_repository,
            category_repository,
        }
    }

    pub async fn execute(&self, command: UpdateProductCommand) -> anyhow::Result<ProductDto> {
        let category = self.category_repository.find_by_id(command.category_id())?;
        let product =
            ProductMapper::map_update_product_command_to_domain_entity(&command, category)?;

        self.product_repository.save(product).map(ProductDto::from)
    }
}

pub struct DeleteProductCommandHandler {
    repository: Arc<dyn ProductRepository>,
}

impl DeleteProductCommandHandler {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: DeleteProductCommand) -> anyhow::Result<()> {
        self.repository.delete(command.id)
    }
}
