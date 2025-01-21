use najm_course_api::libs::database::schemas::{
    app_permissions_schema, app_roles_permissions_schema, app_roles_schema, app_users_schema,
};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create `app_roles` table
        manager
            .create_table(
                Table::create()
                    .table(app_roles_schema::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(app_roles_schema::Column::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(app_roles_schema::Column::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(app_roles_schema::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(app_roles_schema::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create `app_users` table
        manager
            .create_table(
                Table::create()
                    .table(app_users_schema::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(app_users_schema::Column::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(app_users_schema::Column::RoleId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(app_users_schema::Column::Fullname)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(app_users_schema::Column::Email)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(app_users_schema::Column::EmailVerified)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(app_users_schema::Column::ReferralCode)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(app_users_schema::Column::ReferredBy)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(app_users_schema::Column::PhoneNumber)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(app_users_schema::Column::Password)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(app_users_schema::Column::Avatar)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(app_users_schema::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(app_users_schema::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_app_users_role_id")
                            .from(app_users_schema::Entity, app_users_schema::Column::RoleId)
                            .to(app_roles_schema::Entity, app_roles_schema::Column::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create `app_permissions` table
        manager
            .create_table(
                Table::create()
                    .table(app_permissions_schema::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(app_permissions_schema::Column::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(app_permissions_schema::Column::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(app_permissions_schema::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(app_permissions_schema::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create `app_roles_permissions` table
        manager
            .create_table(
                Table::create()
                    .table(app_roles_permissions_schema::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(app_roles_permissions_schema::Column::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(app_roles_permissions_schema::Column::RoleId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(app_roles_permissions_schema::Column::PermissionId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_app_roles_permissions_role_id")
                            .from(
                                app_roles_permissions_schema::Entity,
                                app_roles_permissions_schema::Column::RoleId,
                            )
                            .to(app_roles_schema::Entity, app_roles_schema::Column::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_app_roles_permissions_permission_id")
                            .from(
                                app_roles_permissions_schema::Entity,
                                app_roles_permissions_schema::Column::PermissionId,
                            )
                            .to(
                                app_permissions_schema::Entity,
                                app_permissions_schema::Column::Id,
                            ),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop `app_roles_permissions` table
        manager
            .drop_table(
                Table::drop()
                    .table(app_roles_permissions_schema::Entity)
                    .to_owned(),
            )
            .await?;

        // Drop `app_permissions` table
        manager
            .drop_table(
                Table::drop()
                    .table(app_permissions_schema::Entity)
                    .to_owned(),
            )
            .await?;

        // Drop `app_users` table
        manager
            .drop_table(Table::drop().table(app_users_schema::Entity).to_owned())
            .await?;

        // Drop `app_roles` table
        manager
            .drop_table(Table::drop().table(app_roles_schema::Entity).to_owned())
            .await?;

        Ok(())
    }
}
