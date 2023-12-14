use std::{collections::HashMap, pin::Pin};

use futures::Future;
use serde::Deserialize;
use serde_json::Value;

use crate::{resources::RpcResources, Error, Result};

pub use self::{
	from_resources::FromResources,
	into_params::{IntoDefaultParams, IntoParams},
	rpc_handler::RpcHandler,
	rpc_handler_wrapper::{RpcHandlerWrapper, RpcHandlerWrapperTrait},
};

mod from_resources;
mod into_params;
mod rpc_handler;
mod rpc_handler_wrapper;

// -- The raw JSON-RPC request object, serving as the foundation for RPC routing.
#[derive(Deserialize)]
pub struct RpcRequest {
	pub id: Option<Value>,
	pub method: String,
	pub params: Option<Value>,
}

pub type PinFutureValue = Pin<Box<dyn Future<Output = Result<Value>> + Send>>;

// -- method, which calls the appropriate handler matching the method_name.
// -- RpcRouter can be extended with other RpcRouters for composability.
pub struct RpcRouter {
	route_by_name: HashMap<&'static str, Box<dyn RpcHandlerWrapperTrait>>,
}

impl RpcRouter {
	pub fn new() -> Self {
		Self {
			route_by_name: HashMap::new(),
		}
	}

	pub fn add_dyn(
		mut self,
		name: &'static str,
		dyn_handler: Box<dyn RpcHandlerWrapperTrait>,
	) -> Self {
		self.route_by_name.insert(name, dyn_handler);
		self
	}

	pub fn add<F, T, P, R>(self, name: &'static str, handler: F) -> Self
	where
		F: RpcHandler<T, P, R> + Clone + Send + Sync + 'static,
		T: Send + Sync + 'static,
		P: Send + Sync + 'static,
		R: Send + Sync + 'static,
	{
		self.add_dyn(name, handler.into_dyn())
	}

	pub fn extend(mut self, other_router: RpcRouter) -> Self {
		self.route_by_name.extend(other_router.route_by_name);
		self
	}

	pub async fn call(
		&self,
		method: &str,
		rpc_resources: RpcResources,
		params: Option<Value>,
	) -> Result<Value> {
		if let Some(route) = self.route_by_name.get(method) {
			route.call(rpc_resources, params).await
		} else {
			Err(Error::RpcMethodUnknown(method.to_string()))
		}
	}
}

#[macro_export]
macro_rules! rpc_router {
	($($fn_name:ident),+ $(,)?) => {
		{
			use $crate::router::{RpcHandler, RpcRouter};

			let mut router = RpcRouter::new();
			$(
				router = router.add_dyn(stringify!($fn_name), $fn_name.into_dyn());
			)+
			router
		}
	};
}
