use std::net::SocketAddr;

use axum::{middleware, Router};
use error::Result;
use model::ModelManager;
use tower_cookies::CookieManagerLayer;
use web::{
	mw_auth::mw_ctx_resolve, mw_res_map::mw_response_map, routes_login,
	routes_static,
};

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize ModelManager.
	let mm = ModelManager::new().await?;

	// -- Define Routes
	// let routes_rpc = rpc::routes(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));

	let routes_all = Router::new()
		.merge(routes_login::routes())
		// .nest("/api", routes_rpc)
		.layer(middleware::map_response(mw_response_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir());

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	println!("->> {:<12} - {addr}\n", "LISTENING");
	axum::Server::bind(&addr)
		.serve(routes_all.into_make_service())
		.await
		.unwrap();

	Ok(())
}
