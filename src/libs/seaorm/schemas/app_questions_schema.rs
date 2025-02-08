use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "app_questions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: Uuid,
    pub question_text: String,
    pub difficulty_level: Option<f64>,
    pub discrimination: Option<f64>,
    pub guessing: Option<f64>,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::app_options_schema::Entity")]
    Options,
    #[sea_orm(has_many = "super::app_tests_responses_schema::Entity")]
    TestResponses,
}

impl Related<super::app_options_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Options.def()
    }
}

impl Related<super::app_tests_responses_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TestResponses.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
