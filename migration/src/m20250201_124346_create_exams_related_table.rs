use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

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

#[derive(Iden)]
enum AppTests {
    Table,
    Id,
    Name,
    Description,
    Instructions,
    TimeLimit,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum AppQuestions {
    Table,
    Id,
    QuestionText,
    DifficultyLevel,
    Discrimination,
    Guessing,
    Category,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum AppOptions {
    Table,
    Id,
    QuestionId,
    OptionText,
    IsCorrect,
}

#[derive(Iden)]
enum AppTestsSessions {
    Table,
    Id,
    UserId,
    TestId,
    StartTime,
    EndTime,
    Score,
}

#[derive(Iden)]
enum AppTestsResponses {
    Table,
    ResponseId,
    SessionId,
    QuestionId,
    OptionId,
    AnswerTime,
    IsCorrect,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AppRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AppRoles::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AppRoles::Name).string().not_null())
                    .col(ColumnDef::new(AppRoles::CreatedAt).timestamp().null())
                    .col(ColumnDef::new(AppRoles::UpdatedAt).timestamp().null())
                    .to_owned(),
            )
            .await?;

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

        manager
            .create_table(
                Table::create()
                    .table(AppUsers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AppUsers::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AppUsers::RoleId).uuid().not_null())
                    .col(ColumnDef::new(AppUsers::Fullname).string().not_null())
                    .col(
                        ColumnDef::new(AppUsers::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(AppUsers::EmailVerified).timestamp().null())
                    .col(ColumnDef::new(AppUsers::ReferralCode).string().null())
                    .col(ColumnDef::new(AppUsers::ReferredBy).string().null())
                    .col(ColumnDef::new(AppUsers::PhoneNumber).string().not_null())
                    .col(ColumnDef::new(AppUsers::Password).string().not_null())
                    .col(ColumnDef::new(AppUsers::Avatar).string().null())
                    .col(ColumnDef::new(AppUsers::BirthDate).timestamp().null())
                    .col(ColumnDef::new(AppUsers::Gender).string().null())
                    .col(ColumnDef::new(AppUsers::Religion).string().null())
                    .col(ColumnDef::new(AppUsers::IdentityNumber).string().null())
                    .col(ColumnDef::new(AppUsers::StudentType).string().not_null())
                    .col(ColumnDef::new(AppUsers::IsActive).boolean().not_null())
                    .col(ColumnDef::new(AppUsers::IsDeleted).boolean().not_null())
                    .col(
                        ColumnDef::new(AppUsers::IsProfileCompleted)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AppUsers::CreatedAt).timestamp().null())
                    .col(ColumnDef::new(AppUsers::UpdatedAt).timestamp().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-app_users-role_id")
                            .from(AppUsers::Table, AppUsers::RoleId)
                            .to(AppRoles::Table, AppRoles::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AppTests::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AppTests::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AppTests::Name).string().not_null())
                    .col(ColumnDef::new(AppTests::Description).string().null())
                    .col(ColumnDef::new(AppTests::Instructions).string().null())
                    .col(ColumnDef::new(AppTests::TimeLimit).integer().null())
                    .col(ColumnDef::new(AppTests::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(AppTests::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AppQuestions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AppQuestions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AppQuestions::QuestionText)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AppQuestions::DifficultyLevel)
                            .double()
                            .null(),
                    )
                    .col(ColumnDef::new(AppQuestions::Discrimination).double().null())
                    .col(ColumnDef::new(AppQuestions::Guessing).double().null())
                    .col(ColumnDef::new(AppQuestions::Category).string().null())
                    .col(
                        ColumnDef::new(AppQuestions::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AppQuestions::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AppOptions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AppOptions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AppOptions::QuestionId).uuid().not_null())
                    .col(ColumnDef::new(AppOptions::OptionText).string().not_null())
                    .col(ColumnDef::new(AppOptions::IsCorrect).boolean().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-options-question_id")
                            .from(AppOptions::Table, AppOptions::QuestionId)
                            .to(AppQuestions::Table, AppQuestions::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AppTestsSessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AppTestsSessions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AppTestsSessions::UserId).uuid().not_null())
                    .col(ColumnDef::new(AppTestsSessions::TestId).uuid().not_null())
                    .col(
                        ColumnDef::new(AppTestsSessions::StartTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AppTestsSessions::EndTime).timestamp().null())
                    .col(ColumnDef::new(AppTestsSessions::Score).double().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-app_tests_sessions-user_id")
                            .from(AppTestsSessions::Table, AppTestsSessions::UserId)
                            .to(AppUsers::Table, AppUsers::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-app_tests_sessions-test_id")
                            .from(AppTestsSessions::Table, AppTestsSessions::TestId)
                            .to(AppTests::Table, AppTests::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AppTestsResponses::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AppTestsResponses::ResponseId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AppTestsResponses::SessionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AppTestsResponses::QuestionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AppTestsResponses::OptionId).uuid().null())
                    .col(
                        ColumnDef::new(AppTestsResponses::AnswerTime)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(AppTestsResponses::IsCorrect)
                            .boolean()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-app_tests_responses-session_id")
                            .from(AppTestsResponses::Table, AppTestsResponses::SessionId)
                            .to(AppTestsSessions::Table, AppTestsSessions::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-app_tests_responses-question_id")
                            .from(AppTestsResponses::Table, AppTestsResponses::QuestionId)
                            .to(AppQuestions::Table, AppQuestions::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-app_tests_responses-option_id")
                            .from(AppTestsResponses::Table, AppTestsResponses::OptionId)
                            .to(AppOptions::Table, AppOptions::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AppTestsResponses::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AppTestsSessions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AppOptions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AppQuestions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AppTests::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AppUsers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AppRoles::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AppPermissions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AppRolesPermissions::Table).to_owned())
            .await?;
        Ok(())
    }
}
