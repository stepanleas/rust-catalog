use crate::api::api_products::__path_create;
use crate::api::api_products::__path_delete;
use crate::api::api_products::__path_find_one;
use crate::api::api_products::__path_list_all;
use crate::api::api_products::__path_update;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Product", description = "Product management endpoints.")
    ),
    paths(
        list_all,
        find_one,
        create,
        update,
        delete,
    )
)]
pub(crate) struct ProductApiDoc;
