use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_tests")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: Uuid,
	pub test_name: String,
	pub session_id: Uuid,
	pub created_at: Option<DateTimeUtc>,
	pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::app_test_sessions_schema::Entity",
		from = "Column::SessionId",
		to = "super::app_test_sessions_schema::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Session,
}

impl Related<super::app_test_sessions_schema::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Session.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
