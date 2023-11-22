use crate::{crypt, model, web};
use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	// -- Login
	LoginFailUsernameNotFound,
	LoginFailUserHasNoPwd { user_id: i64 },
	LoginFailPwdNotMatching { user_id: i64 },

	// -- CtxExtError
	CtxExt(web::mw_auth::CtxExtError),

	// -- Modules
	Model(model::Error),
	Crypt(crypt::Error),
}

impl From<model::Error> for Error {
	fn from(val: model::Error) -> Self {
		Self::Model(val)
	}
}

impl From<crypt::Error> for Error {
	fn from(val: crypt::Error) -> Self {
		Self::Crypt(val)
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		debug!(" {:12} - {self:?}", "INTO_RES");

		// Create a placeholder Axum response.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the response.
		response.extensions_mut().insert(self);

		response
	}
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

#[allow(unreachable_patterns)]
impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		use web::Error::*;

		match self {
			// -- Login
			LoginFailUsernameNotFound
			| LoginFailUserHasNoPwd { .. }
			| LoginFailPwdNotMatching { .. } => {
				(StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
			}
			// -- Auth
			CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

			// -- Fallback
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	SERVICE_ERROR,
}
