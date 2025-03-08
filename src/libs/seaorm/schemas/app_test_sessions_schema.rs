use super::app_sessions_has_tests_schema::{self, Relation};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_test_sessions")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: Uuid,
	pub session_name: String,
	pub is_active: bool,
	pub description: String,
	pub category: String,
	pub student_type: Option<String>,
	pub created_at: Option<DateTimeUtc>,
	pub updated_at: Option<DateTimeUtc>,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<app_sessions_has_tests_schema::Entity> for Entity {
	fn to() -> RelationDef {
		app_sessions_has_tests_schema::Relation::Session.def()
	}
}
