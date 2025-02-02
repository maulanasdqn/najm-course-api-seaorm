use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "options")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: Uuid,
    pub question_id: Uuid,
    pub option_text: String,
    pub is_correct: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::app_questions_schema::Entity",
        from = "Column::QuestionId",
        to = "super::app_questions_schema::Column::Id"
    )]
    Question,
    #[sea_orm(has_many = "super::app_tests_responses_schema::Entity")]
    TestResponses,
}

impl Related<super::app_questions_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Question.def()
    }
}

impl Related<super::app_tests_responses_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TestResponses.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
