use crate::api::docs::category_api_doc::CategoryApiDoc;
use crate::api::docs::health_check_api_doc::HealthCheckApiDoc;
use crate::api::docs::product_api_doc::ProductApiDoc;
use utoipa::OpenApi;
use utoipa::openapi::OpenApi as OpenApiStruct;

pub fn open_api_docs() -> OpenApiStruct {
    let mut openapi = ProductApiDoc::openapi();

    openapi.merge(CategoryApiDoc::openapi());
    openapi.merge(HealthCheckApiDoc::openapi());

    openapi
}
