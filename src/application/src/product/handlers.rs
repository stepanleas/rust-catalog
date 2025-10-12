use crate::product::commands::{CreateProductCommand, DeleteProductCommand, UpdateProductCommand};
use crate::product::dtos::ProductDto;
use crate::product::repositories::ProductRepository;
use crate::{CategoryRepository, FindProductQuery};
use domain::Product;
use shared::domain::value_objects::Money;
use std::sync::Arc;
use uuid::Uuid;

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
}

impl CreateProductCommandHandler {
    pub fn new(
        product_repository: Arc<dyn ProductRepository>,
        category_repository: Arc<dyn CategoryRepository>,
    ) -> Self {
        Self {
            product_repository,
            category_repository,
        }
    }

    pub async fn execute(&self, command: CreateProductCommand) -> anyhow::Result<ProductDto> {
        let category = self.category_repository.find_by_id(command.category_id())?;
        let price = Money::from_f64(command.price())?;

        let product = Product::builder()
            .id(Uuid::new_v4())
            .title(command.title())
            .description(command.description())
            .quantity(command.quantity())
            .price(price)
            .category(category)
            .build();

        self.product_repository.save(product).map(ProductDto::from)
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
        let price = Money::from_f64(command.price())?;

        let product = Product::builder()
            .id(command.id())
            .title(command.title())
            .description(command.description())
            .quantity(command.quantity())
            .price(price)
            .category(category)
            .build();

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
