use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_user_question_answers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub answer_id: Uuid,
    pub question_id: Uuid,
    pub option_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::app_user_answers_schema::Entity",
        from = "Column::AnswerId",
        to = "crate::app_user_answers_schema::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ParentAnswer,
    #[sea_orm(
        belongs_to = "crate::app_questions_schema::Entity",
        from = "Column::QuestionId",
        to = "crate::app_questions_schema::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Question,
    #[sea_orm(
        belongs_to = "crate::app_options_schema::Entity",
        from = "Column::OptionId",
        to = "crate::app_options_schema::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Option,
}

impl ActiveModelBehavior for ActiveModel {}
