use crate::app_sessions_has_tests_schema::Relation;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::app_sessions_has_tests_schema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_tests")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: Uuid,
	pub test_name: String,
	pub created_at: Option<DateTimeUtc>,
	pub updated_at: Option<DateTimeUtc>,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<app_sessions_has_tests_schema::Entity> for Entity {
	fn to() -> RelationDef {
		app_sessions_has_tests_schema::Relation::Test.def()
	}
}
