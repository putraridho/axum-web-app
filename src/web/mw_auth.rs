use crate::{
	ctx::Ctx,
	model::ModelManager,
	web::{Error, Result},
};
use async_trait::async_trait;
use axum::{
	extract::{FromRequestParts, State},
	http::{request::Parts, Request},
	middleware::Next,
	response::Response,
};
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use super::AUTH_TOKEN;

#[allow(dead_code)]
pub async fn mw_ctx_require<B>(
	ctx: Result<Ctx>,
	req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	debug!(" {:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

	ctx?;

	Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve<B>(
	_mc: State<ModelManager>,
	cookies: Cookies,
	mut req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	debug!(" {:<12} - mw_ctx_resolve", "MIDDLEWARE");

	let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

	// FIXME - Compute real CtxAuthResult<Ctx>.
	let result_ctx =
		Ctx::new(100).map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()));

	// Remove the cookie if something went wrong other than NoAuthTokenCookie.
	if result_ctx.is_err()
		&& !matches!(result_ctx, Err(CtxExtError::TokenNotInCookie))
	{
		cookies.remove(Cookie::named(AUTH_TOKEN))
	}

	// Store the ctx_result in the request extension.
	req.extensions_mut().insert(result_ctx);

	Ok(next.run(req).await)
}

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;
	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		debug!(" {:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<CtxExtResult>()
			.ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
			.clone()
			.map_err(Error::CtxExt)
	}
}
// endregion: --- Ctx Extractor

type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Debug, Clone, Serialize)]
pub enum CtxExtError {
	TokenNotInCookie,
	CtxNotInRequestExt,
	CtxCreateFail(String),
}
