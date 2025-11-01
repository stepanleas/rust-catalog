use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[readonly::make]
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateProductRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,

    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,

    #[validate(range(min = 0, message = "Quantity must be greater or equal to 0"))]
    pub quantity: i32,

    #[validate(range(min = 1.0, message = "Price must be greater than 0"))]
    pub price: f64,

    // TODO: check validation for uuid
    pub category_id: Uuid,
}

#[readonly::make]
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProductRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,

    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,

    #[validate(range(min = 0, message = "Quantity must be greater or equal to 0"))]
    pub quantity: i32,

    #[validate(range(min = 1.0, message = "Price must be greater than 0"))]
    pub price: f64,

    // TODO: check validation for uuid
    pub category_id: Uuid,
}
