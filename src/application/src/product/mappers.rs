use crate::{CreateProductCommand, UpdateProductCommand};
use domain::{Category, Product};
use shared::domain::value_objects::{Money, ProductId};

pub struct ProductMapper;

impl ProductMapper {
    pub fn map_create_product_command_to_domain_entity(
        command: &CreateProductCommand,
        category: Category,
    ) -> anyhow::Result<Product> {
        let price = Money::from_f64(command.price())?;

        let product = Product::builder()
            .id(ProductId::new())
            .title(command.title())
            .description(command.description())
            .quantity(command.quantity())
            .price(price)
            .category(category)
            .build();

        Ok(product)
    }

    pub fn map_update_product_command_to_domain_entity(
        command: &UpdateProductCommand,
        category: Category,
    ) -> anyhow::Result<Product> {
        let price = Money::from_f64(command.price())?;

        let product = Product::builder()
            .id(ProductId::from_uuid(command.id()))
            .title(command.title())
            .description(command.description())
            .quantity(command.quantity())
            .price(price)
            .category(category)
            .build();

        Ok(product)
    }
}
