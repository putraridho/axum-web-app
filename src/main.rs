mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;
// #[cfg(test)] // Commented during early development
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;

use crate::{
	model::ModelManager,
	web::{
		mw_auth::{mw_ctx_require, mw_ctx_resolve},
		mw_res_map::mw_response_map,
		routes_login, routes_static, rpc,
	},
};
use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.without_time()
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// FOR DEV ONLY
	_dev_utils::init_dev().await;

	// Initialize ModelManager.
	let mm = ModelManager::new().await?;

	// -- Define Routes
	let routes_rpc =
		rpc::routes(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));

	let routes_all = Router::new()
		.merge(routes_login::routes(mm.clone()))
		.nest("/api", routes_rpc)
		.layer(middleware::map_response(mw_response_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir());

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	info!("{:<12} - {addr}\n", "LISTENING");
	axum::Server::bind(&addr)
		.serve(routes_all.into_make_service())
		.await
		.unwrap();

	Ok(())
}
