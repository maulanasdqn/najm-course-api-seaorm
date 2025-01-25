use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub role_id: Uuid,
    pub fullname: String,
    pub email: String,
    pub email_verified: Option<DateTimeUtc>,
    pub referral_code: Option<String>,
    pub referred_by: Option<String>,
    pub phone_number: String,
    pub password: String,
    pub avatar: Option<String>,
    pub birth_date: Option<DateTimeUtc>,
    pub gender: Option<String>,
    pub religion: Option<String>,
    pub identity_number: Option<String>,
    pub student_type: String,
    pub is_active: bool,
    pub is_deleted: bool,
    pub is_profile_completed: bool,
    pub otp: Option<String>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::app_roles_schema::Entity",
        from = "Column::RoleId",
        to = "super::app_roles_schema::Column::Id"
    )]
    Role,
}

impl Related<super::app_roles_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
