mod base;
mod error;
pub mod modql_utils;
pub mod project;
mod store;
pub mod task;
pub mod user;

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	// Constructore
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;
		Ok(ModelManager { db })
	}

	// Returns the sqlx db pool reference.
	// (Only for the model layer)
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
