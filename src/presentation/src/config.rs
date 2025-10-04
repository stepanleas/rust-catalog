use crate::api;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/categories")
            .service(api::list_all_categories)
            .service(api::find_one_category)
            .service(api::create_category)
            .service(api::update_category)
            .service(api::delete_category),
    );
    cfg.service(
        web::scope("/api/products")
            .service(api::list_all_products)
            .service(api::find_one_product)
            .service(api::create_product)
            .service(api::update_product)
            .service(api::delete_product),
    );
    cfg.service(
        web::scope("/api/health")
            .service(api::startup)
            .service(api::ready)
            .service(api::live),
    );
    cfg.service(web::scope("/api/info").service(api::info));
}
