use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_questions")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: Uuid,
	pub test_id: Uuid,
	pub question: String,
	pub discussion: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::app_options_schema::Entity")]
	Options,
	#[sea_orm(
		belongs_to = "super::app_tests_schema::Entity",
		from = "Column::TestId",
		to = "super::app_tests_schema::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Test,
}

impl Related<super::app_options_schema::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Options.def()
	}
}

impl Related<super::app_tests_schema::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Test.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
