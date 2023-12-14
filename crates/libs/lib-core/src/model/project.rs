use lib_utils::time::Rfc3339;
use modql::{
	field::Fields,
	filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue},
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

use crate::ctx::Ctx;

#[allow(unused)]
use super::modql_utils::time_to_sea_value;
use super::{
	base::{self, DbBmc},
	ModelManager, Result,
};

#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Project {
	pub id: i64,
	pub owner_id: i64,
	pub name: String,
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct ProjectForCreate {
	pub name: String,
}

#[derive(Fields, Deserialize)]
pub struct ProjectForUpdate {
	pub name: Option<String>,
	pub ownder_id: Option<i64>,
}

#[derive(Fields)]
pub struct ProjectForCreateInner {
	pub name: String,
	pub owner_id: i64,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct ProjectFilter {
	id: Option<OpValsInt64>,
	name: Option<OpValsString>,
	cid: Option<OpValsInt64>,
	#[modql(to_sea_value = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

pub struct ProjectBmc;

impl DbBmc for ProjectBmc {
	const TABLE: &'static str = "project";
}

impl ProjectBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		project_c: ProjectForCreate,
	) -> Result<i64> {
		let project_c = ProjectForCreateInner {
			name: project_c.name,
			owner_id: ctx.user_id(),
		};

		base::create::<Self, _>(ctx, mm, project_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Project> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<ProjectFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Project>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		project_u: ProjectForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, project_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
