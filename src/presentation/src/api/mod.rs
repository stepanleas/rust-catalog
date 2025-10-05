mod api_categories;
mod api_health_check;
mod api_products;

mod api_info;
pub mod docs;

pub use api_categories::create as create_category;
pub use api_categories::delete as delete_category;
pub use api_categories::find_one as find_one_category;
pub use api_categories::list_all as list_all_categories;
pub use api_categories::update as update_category;

pub use api_products::create as create_product;
pub use api_products::delete as delete_product;
pub use api_products::find_one as find_one_product;
pub use api_products::list_all as list_all_products;
pub use api_products::update as update_product;

pub use api_health_check::live;
pub use api_health_check::ready;
pub use api_health_check::startup;

pub use api_info::info;
