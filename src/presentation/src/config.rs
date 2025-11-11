use crate::api;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/categories")
            .service(api::api_categories::list_all)
            .service(api::api_categories::find_one)
            .service(api::api_categories::create)
            .service(api::api_categories::update)
            .service(api::api_categories::delete),
    );
    cfg.service(
        web::scope("/api/products")
            .service(api::api_products::list_all)
            .service(api::api_products::find_one)
            .service(api::api_products::create)
            .service(api::api_products::update)
            .service(api::api_products::delete),
    );
    cfg.service(
        web::scope("/api/health")
            .service(api::api_health_check::startup)
            .service(api::api_health_check::ready)
            .service(api::api_health_check::live),
    );
    cfg.service(web::scope("/api/info").service(api::api_info::info));
}
