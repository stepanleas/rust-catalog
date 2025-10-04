use crate::app_state::AppState;
use crate::error::ApiError;
use crate::requests::{CreateCategoryRequest, UpdateCategoryRequest};
use crate::responses::CategoryResponse;
use crate::validation::ValidatedJson;
use actix_web::web::Path;
use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, put, web};
use anyhow::anyhow;
use application::{
    CreateCategoryCommand, CreateCategoryCommandHandler, DeleteCategoryCommand,
    DeleteCategoryCommandHandler, FindCategoryQuery, FindCategoryQueryHandler,
    ListAllCategoryQueryHandler, UpdateCategoryCommand, UpdateCategoryCommandHandler,
};
use serde_json::json;
use uuid::Uuid;

const CATEGORIES: &str = "Categories";

#[utoipa::path(
    context_path = "/api/categories",
    tag = CATEGORIES,
    responses(
        (status = 200, description = "List current category items", body = [CategoryResponse])
    )
)]
#[get("")]
pub async fn list_all(req: HttpRequest) -> Result<impl Responder, ApiError> {
    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = ListAllCategoryQueryHandler::new(state.category_repository.clone());

    let categories = handler.execute().await?;
    let data = categories
        .into_iter()
        .map(CategoryResponse::from)
        .collect::<Vec<CategoryResponse>>();

    Ok(HttpResponse::Ok().json(json!({ "data": data })))
}

#[utoipa::path(
    context_path = "/api/categories",
    tag = CATEGORIES,
    responses(
        (status = 200, description = "Get category item by id", body = [CategoryResponse])
    ),
    params(
        ("id" = Uuid, Path, description = "Id of the category item")
    ),
)]
#[get("/{id}")]
pub async fn find_one(req: HttpRequest, id: Path<Uuid>) -> Result<impl Responder, ApiError> {
    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = FindCategoryQueryHandler::new(state.category_repository.clone());

    let category = handler
        .execute(FindCategoryQuery::new(Some(id.into_inner())))
        .await?;

    Ok(HttpResponse::Ok().json(json!({ "data": CategoryResponse::from(category) })))
}

#[utoipa::path(
    context_path = "/api/categories",
    tag = CATEGORIES,
    responses(
        (status = 201, description = "Create a category item", body = [CategoryResponse])
    ),
    request_body = CreateCategoryRequest,
)]
#[post("")]
pub async fn create(
    req: HttpRequest,
    request: ValidatedJson<CreateCategoryRequest>,
) -> Result<impl Responder, ApiError> {
    let payload = request.into_inner();

    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = CreateCategoryCommandHandler::new(state.category_repository.clone());
    let command = CreateCategoryCommand::new(payload.title.clone(), payload.description.clone());

    let category = handler.execute(command).await?;

    Ok(HttpResponse::Created().json(json!({ "data": CategoryResponse::from(category) })))
}

#[utoipa::path(
    context_path = "/api/categories",
    tag = CATEGORIES,
    responses(
        (status = 200, description = "Update a category item", body = [CategoryResponse])
    ),
    params(
        ("id", description = "Id of the category item to update")
    ),
    request_body = UpdateCategoryRequest,
)]
#[put("/{id}")]
pub async fn update(
    req: HttpRequest,
    id: Path<Uuid>,
    request: ValidatedJson<UpdateCategoryRequest>,
) -> Result<impl Responder, ApiError> {
    let payload = request.into_inner();

    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = UpdateCategoryCommandHandler::new(state.category_repository.clone());
    let command = UpdateCategoryCommand::new(
        id.into_inner(),
        payload.title.clone(),
        payload.description.clone(),
    );

    let category = handler.execute(command).await?;

    Ok(HttpResponse::Created().json(json!({ "data": CategoryResponse::from(category) })))
}

#[utoipa::path(
    context_path = "/api/categories",
    tag = CATEGORIES,
    responses(
        (status = 204, description = "Delete a category item", body = [CategoryResponse])
    ),
    params(
        ("id", description = "Id of the category item to delete")
    )
)]
#[delete("/{id}")]
pub async fn delete(req: HttpRequest, id: Path<Uuid>) -> Result<impl Responder, ApiError> {
    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = DeleteCategoryCommandHandler::new(state.category_repository.clone());
    let command = DeleteCategoryCommand::new(id.into_inner());

    handler.execute(command).await?;

    Ok(HttpResponse::NoContent().finish())
}
