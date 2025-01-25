use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create app_roles table
        manager
            .create_table(
                Table::create()
                    .table(AppRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AppRoles::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AppRoles::Name).string().not_null())
                    .col(ColumnDef::new(AppRoles::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(AppRoles::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // Create app_permissions table
        manager
            .create_table(
                Table::create()
                    .table(AppPermissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AppPermissions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AppPermissions::Name).string().not_null())
                    .col(ColumnDef::new(AppPermissions::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(AppPermissions::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // Create app_roles_permissions table
        manager
            .create_table(
                Table::create()
                    .table(AppRolesPermissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AppRolesPermissions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AppRolesPermissions::PermissionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AppRolesPermissions::RoleId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-roles_permissions-permission_id")
                            .from(
                                AppRolesPermissions::Table,
                                AppRolesPermissions::PermissionId,
                            )
                            .to(AppPermissions::Table, AppPermissions::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-roles_permissions-role_id")
                            .from(AppRolesPermissions::Table, AppRolesPermissions::RoleId)
                            .to(AppRoles::Table, AppRoles::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create app_users table
        manager
            .create_table(
                Table::create()
                    .table(AppUsers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AppUsers::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AppUsers::RoleId).uuid().not_null())
                    .col(ColumnDef::new(AppUsers::Fullname).string().not_null())
                    .col(ColumnDef::new(AppUsers::Email).string().not_null())
                    .col(ColumnDef::new(AppUsers::EmailVerified).timestamp_with_time_zone())
                    .col(ColumnDef::new(AppUsers::ReferralCode).string())
                    .col(ColumnDef::new(AppUsers::ReferredBy).string())
                    .col(ColumnDef::new(AppUsers::PhoneNumber).string().not_null())
                    .col(ColumnDef::new(AppUsers::Password).string().not_null())
                    .col(ColumnDef::new(AppUsers::Avatar).string())
                    .col(ColumnDef::new(AppUsers::BirthDate).timestamp_with_time_zone())
                    .col(ColumnDef::new(AppUsers::Gender).string())
                    .col(ColumnDef::new(AppUsers::Religion).string())
                    .col(ColumnDef::new(AppUsers::IdentityNumber).string())
                    .col(ColumnDef::new(AppUsers::StudentType).string().not_null())
                    .col(ColumnDef::new(AppUsers::IsActive).boolean().not_null())
                    .col(ColumnDef::new(AppUsers::IsDeleted).boolean().not_null())
                    .col(ColumnDef::new(AppUsers::Otp).string()
                    .col(
                        ColumnDef::new(AppUsers::IsProfileCompleted)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AppUsers::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(AppUsers::UpdatedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-users-role_id")
                            .from(AppUsers::Table, AppUsers::RoleId)
                            .to(AppRoles::Table, AppRoles::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order of creation to satisfy foreign key constraints
        manager
            .drop_table(Table::drop().table(AppUsers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AppRolesPermissions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AppPermissions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AppRoles::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum AppUsers {
    Table,
    Id,
    RoleId,
    Fullname,
    Email,
    EmailVerified,
    ReferralCode,
    ReferredBy,
    PhoneNumber,
    Password,
    Avatar,
    BirthDate,
    Gender,
    Religion,
    IdentityNumber,
    StudentType,
    IsActive,
    IsDeleted,
    IsProfileCompleted,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum AppRoles {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum AppPermissions {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum AppRolesPermissions {
    Table,
    Id,
    PermissionId,
    RoleId,
}
