use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[readonly::make]
#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateCategoryRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,

    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,
}

#[readonly::make]
#[derive(Deserialize, Validate, ToSchema)]
pub struct UpdateCategoryRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,

    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,
}
