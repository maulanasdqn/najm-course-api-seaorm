use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "app_tests_responses")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub response_id: Uuid,
    pub session_id: Uuid,
    pub question_id: Uuid,
    pub option_id: Option<Uuid>,
    pub answer_time: Option<DateTime<Utc>>,
    pub is_correct: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::app_tests_sessions_schema::Entity",
        from = "Column::SessionId",
        to = "super::app_tests_sessions_schema::Column::Id"
    )]
    TestSession,

    #[sea_orm(
        belongs_to = "super::app_questions_schema::Entity",
        from = "Column::QuestionId",
        to = "super::app_questions_schema::Column::Id"
    )]
    Question,

    #[sea_orm(
        belongs_to = "super::app_options_schema::Entity",
        from = "Column::OptionId",
        to = "super::app_options_schema::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Option,
}

impl Related<super::app_options_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Option.def()
    }
}

impl Related<super::app_questions_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Question.def()
    }
}

impl Related<super::app_tests_sessions_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TestSession.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
