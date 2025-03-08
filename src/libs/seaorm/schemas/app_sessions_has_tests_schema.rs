use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{app_test_sessions_schema, app_tests_schema};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_sessions_has_tests")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: Uuid,
	pub session_id: Uuid,
	pub test_id: Uuid,
	pub start_date: Option<DateTimeUtc>,
	pub end_date: Option<DateTimeUtc>,
	pub weight: Option<String>,
	pub multiplier: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "app_test_sessions_schema::Entity",
		from = "Column::SessionId",
		to = "app_test_sessions_schema::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Session,
	#[sea_orm(
		belongs_to = "app_tests_schema::Entity",
		from = "Column::TestId",
		to = "app_tests_schema::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Test,
}

impl Related<app_test_sessions_schema::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Session.def()
	}
}

impl Related<app_tests_schema::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Test.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
