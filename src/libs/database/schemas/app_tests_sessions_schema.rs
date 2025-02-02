use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "app_tests_sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub test_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub score: Option<f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::app_users_schema::Entity",
        from = "Column::UserId",
        to = "super::app_users_schema::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::app_tests_schema::Entity",
        from = "Column::TestId",
        to = "super::app_tests_schema::Column::Id"
    )]
    Test,
    #[sea_orm(has_many = "super::app_tests_responses_schema::Entity")]
    TestResponses,
}

impl Related<super::app_users_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::app_tests_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Test.def()
    }
}

impl Related<super::app_tests_responses_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TestResponses.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
