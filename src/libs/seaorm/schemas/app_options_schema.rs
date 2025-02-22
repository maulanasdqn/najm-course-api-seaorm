use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_options")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: Uuid,
	pub question_id: Uuid,
	pub label: String,
	pub is_correct: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::app_questions_schema::Entity",
		from = "Column::QuestionId",
		to = "super::app_questions_schema::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Question,
}

impl Related<super::app_questions_schema::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Question.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
