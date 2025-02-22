use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_user_answers")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: Uuid,
	pub user_id: Uuid,
	pub test_id: Uuid,
	pub question_id: Uuid,
	pub option_id: Option<Uuid>,
	pub answer: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::app_users_schema::Entity",
		from = "Column::UserId",
		to = "super::app_users_schema::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	User,
	#[sea_orm(
		belongs_to = "super::app_tests_schema::Entity",
		from = "Column::TestId",
		to = "super::app_tests_schema::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Test,
	#[sea_orm(
		belongs_to = "super::app_questions_schema::Entity",
		from = "Column::QuestionId",
		to = "super::app_questions_schema::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Question,
	#[sea_orm(
		belongs_to = "super::app_options_schema::Entity",
		from = "Column::OptionId",
		to = "super::app_options_schema::Column::Id",
		on_update = "Cascade",
		on_delete = "Cascade"
	)]
	Option,
}

impl Related<super::app_users_schema::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::User.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
