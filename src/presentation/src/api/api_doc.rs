use crate::api::api_categories::__path_create;
use crate::api::api_categories::__path_delete;
use crate::api::api_categories::__path_find_one;
use crate::api::api_categories::__path_list_all;
use crate::api::api_categories::__path_update;
use crate::api::api_health_check::__path_live;
use crate::api::api_health_check::__path_ready;
use crate::api::api_health_check::__path_startup;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Catalog", description = "Catalog management endpoints.")
    ),
    paths(
        startup,
        live,
        ready,
        list_all,
        find_one,
        create,
        update,
        delete,
    )
)]
pub struct ApiDoc;
