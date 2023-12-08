use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::pwd;

use super::store;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
	EntityNotFound {
		entity: &'static str,
		id: i64,
	},
	ListLimitOverMax {
		max: i64,
		actual: i64,
	},

	// Modules
	#[from]
	Pwd(pwd::Error),
	#[from]
	Store(store::Error),

	// Externals
	#[from]
	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
	#[from]
	SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),
	#[from]
	ModqlIntoSea(#[serde_as(as = "DisplayFromStr")] modql::filter::IntoSeaError),
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut std::fmt::Formatter<'_>,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
