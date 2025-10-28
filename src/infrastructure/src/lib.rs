use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, r2d2};

mod category;
mod config;
mod product;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub use category::postgres_category_repository::PostgresCategoryRepository;
pub use config::configure;
pub use product::postgres_product_repository::PostgresProductRepository;
