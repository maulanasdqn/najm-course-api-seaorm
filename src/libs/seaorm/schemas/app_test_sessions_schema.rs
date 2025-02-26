use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_test_sessions")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: Uuid,
	pub session_name: String,
	pub student_type: Option<String>,
	pub start_date: DateTimeUtc,
	pub end_date: DateTimeUtc,
	pub created_at: Option<DateTimeUtc>,
	pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		has_many = "super::app_tests_schema::Entity",
		from = "Column::Id",
		to = "super::app_tests_schema::Column::SessionId"
	)]
	Tests,
}

impl Related<super::app_tests_schema::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Tests.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
