use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_roles_permissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub permission_id: Uuid,
    pub role_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::app_permissions_schema::Entity",
        from = "Column::PermissionId",
        to = "super::app_permissions_schema::Column::Id"
    )]
    Permission,
    #[sea_orm(
        belongs_to = "super::app_roles_schema::Entity",
        from = "Column::RoleId",
        to = "super::app_roles_schema::Column::Id"
    )]
    Role,
}

impl Related<super::app_permissions_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}

impl Related<super::app_roles_schema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
