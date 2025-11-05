use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, r2d2};

pub mod category;
pub mod config;
pub mod product;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
