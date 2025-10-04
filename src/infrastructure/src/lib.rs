use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, r2d2};

mod category;
mod config;
mod product;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub use category::category_postgres_repository::PostgresCategoryRepository;
pub use config::configure;
pub use product::product_postgres_repository::PostgresProductRepository;
