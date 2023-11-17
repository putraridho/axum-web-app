use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config;

pub use self::error::{Error, Result};

mod error;

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
	PgPoolOptions::new()
		.max_connections(5)
		.connect(&config().DB_URL)
		.await
		.map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}
