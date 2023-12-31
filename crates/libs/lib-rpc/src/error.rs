use derive_more::From;
use lib_core::model;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	MissingCtx,

	// -- RPC Router
	RpcMethodUnknown(String),
	RpcIntoParamsMissing,

	// -- Modules
	#[from]
	Model(model::Error),

	// -- External Modules
	#[from]
	SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
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
