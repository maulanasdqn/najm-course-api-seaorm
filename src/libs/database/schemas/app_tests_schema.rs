use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "app_tests")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub time_limit: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::app_tests_sessions_schema::Entity")]
    TestSessions,
}

impl ActiveModelBehavior for ActiveModel {}
