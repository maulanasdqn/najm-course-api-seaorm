use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
					.col(
						ColumnDef::new(AppPermissions::CreatedAt)
							.timestamp_with_time_zone(),
					)
					.col(
						ColumnDef::new(AppPermissions::UpdatedAt)
							.timestamp_with_time_zone(),
					)
					.to_owned(),
			)
			.await?;

		manager
			.create_table(
				Table::create()
					.table(AppRoles::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(AppRoles::Id).uuid().not_null().primary_key(),
					)
					.col(ColumnDef::new(AppRoles::Name).string().not_null())
					.col(
						ColumnDef::new(AppRoles::CreatedAt)
							.timestamp_with_time_zone(),
					)
					.col(
						ColumnDef::new(AppRoles::UpdatedAt)
							.timestamp_with_time_zone(),
					)
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
							.name("fk_roles_permissions_permission")
							.from(
								AppRolesPermissions::Table,
								AppRolesPermissions::PermissionId,
							)
							.to(AppPermissions::Table, AppPermissions::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.foreign_key(
						ForeignKey::create()
							.name("fk_roles_permissions_role")
							.from(
								AppRolesPermissions::Table,
								AppRolesPermissions::RoleId,
							)
							.to(AppRoles::Table, AppRoles::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.to_owned(),
			)
			.await?;

		manager
			.create_table(
				Table::create()
					.table(AppTestSessions::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(AppTestSessions::Id)
							.uuid()
							.not_null()
							.primary_key(),
					)
					.col(
						ColumnDef::new(AppTestSessions::SessionName)
							.string()
							.not_null(),
					)
					.col(
						ColumnDef::new(AppTestSessions::StartDate)
							.timestamp_with_time_zone()
							.not_null(),
					)
					.col(
						ColumnDef::new(AppTestSessions::EndDate)
							.timestamp_with_time_zone()
							.not_null(),
					)
					.col(
						ColumnDef::new(AppTestSessions::CreatedAt)
							.timestamp_with_time_zone(),
					)
					.col(
						ColumnDef::new(AppTestSessions::UpdatedAt)
							.timestamp_with_time_zone(),
					)
					.to_owned(),
			)
			.await?;

		manager
			.create_table(
				Table::create()
					.table(AppTests::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(AppTests::Id).uuid().not_null().primary_key(),
					)
					.col(ColumnDef::new(AppTests::TestName).string().not_null())
					.col(ColumnDef::new(AppTests::SessionId).uuid().not_null())
					.col(
						ColumnDef::new(AppTests::CreatedAt)
							.timestamp_with_time_zone(),
					)
					.col(
						ColumnDef::new(AppTests::UpdatedAt)
							.timestamp_with_time_zone(),
					)
					.foreign_key(
						ForeignKey::create()
							.name("fk_tests_session")
							.from(AppTests::Table, AppTests::SessionId)
							.to(AppTestSessions::Table, AppTestSessions::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
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
					.col(ColumnDef::new(AppQuestions::TestId).uuid().not_null())
					.col(ColumnDef::new(AppQuestions::Question).string().not_null())
					.col(
						ColumnDef::new(AppQuestions::Discussion).string().not_null(),
					)
					.foreign_key(
						ForeignKey::create()
							.name("fk_questions_test")
							.from(AppQuestions::Table, AppQuestions::TestId)
							.to(AppTests::Table, AppTests::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
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
					.col(ColumnDef::new(AppOptions::Label).string().not_null())
					.col(ColumnDef::new(AppOptions::IsCorrect).boolean().not_null())
					.foreign_key(
						ForeignKey::create()
							.name("fk_options_question")
							.from(AppOptions::Table, AppOptions::QuestionId)
							.to(AppQuestions::Table, AppQuestions::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.to_owned(),
			)
			.await?;

		manager
			.create_table(
				Table::create()
					.table(AppUsers::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(AppUsers::Id).uuid().not_null().primary_key(),
					)
					.col(ColumnDef::new(AppUsers::RoleId).uuid().not_null())
					.col(ColumnDef::new(AppUsers::Fullname).string().not_null())
					.col(ColumnDef::new(AppUsers::Email).string().not_null())
					.col(
						ColumnDef::new(AppUsers::EmailVerified)
							.timestamp_with_time_zone()
							.null(),
					)
					.col(ColumnDef::new(AppUsers::ReferralCode).string().null())
					.col(ColumnDef::new(AppUsers::ReferredBy).string().null())
					.col(ColumnDef::new(AppUsers::PhoneNumber).string().not_null())
					.col(ColumnDef::new(AppUsers::Password).string().not_null())
					.col(ColumnDef::new(AppUsers::Avatar).string().null())
					.col(
						ColumnDef::new(AppUsers::BirthDate)
							.timestamp_with_time_zone()
							.null(),
					)
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
					.col(
						ColumnDef::new(AppUsers::CreatedAt)
							.timestamp_with_time_zone()
							.null(),
					)
					.col(
						ColumnDef::new(AppUsers::UpdatedAt)
							.timestamp_with_time_zone()
							.null(),
					)
					.foreign_key(
						ForeignKey::create()
							.name("fk_users_role")
							.from(AppUsers::Table, AppUsers::RoleId)
							.to(AppRoles::Table, AppRoles::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.to_owned(),
			)
			.await?;

		manager
			.create_table(
				Table::create()
					.table(AppUserAnswers::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(AppUserAnswers::Id)
							.uuid()
							.not_null()
							.primary_key(),
					)
					.col(ColumnDef::new(AppUserAnswers::UserId).uuid().not_null())
					.col(ColumnDef::new(AppUserAnswers::TestId).uuid().not_null())
					.col(
						ColumnDef::new(AppUserAnswers::QuestionId).uuid().not_null(),
					)
					.col(ColumnDef::new(AppUserAnswers::OptionId).uuid().null())
					.col(ColumnDef::new(AppUserAnswers::Answer).string().null())
					.foreign_key(
						ForeignKey::create()
							.name("fk_user_answers_user")
							.from(AppUserAnswers::Table, AppUserAnswers::UserId)
							.to(AppUsers::Table, AppUsers::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.foreign_key(
						ForeignKey::create()
							.name("fk_user_answers_test")
							.from(AppUserAnswers::Table, AppUserAnswers::TestId)
							.to(AppTests::Table, AppTests::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.foreign_key(
						ForeignKey::create()
							.name("fk_user_answers_question")
							.from(AppUserAnswers::Table, AppUserAnswers::QuestionId)
							.to(AppQuestions::Table, AppQuestions::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.foreign_key(
						ForeignKey::create()
							.name("fk_user_answers_option")
							.from(AppUserAnswers::Table, AppUserAnswers::OptionId)
							.to(AppOptions::Table, AppOptions::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.to_owned(),
			)
			.await?;

		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(AppUserAnswers::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(AppUsers::Table).to_owned())
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
			.drop_table(Table::drop().table(AppTestSessions::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(AppRolesPermissions::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(AppRoles::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(AppPermissions::Table).to_owned())
			.await?;
		Ok(())
	}
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
enum AppRoles {
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
enum AppTestSessions {
	Table,
	Id,
	SessionName,
	StartDate,
	EndDate,
	CreatedAt,
	UpdatedAt,
}

#[derive(Iden)]
enum AppTests {
	Table,
	Id,
	TestName,
	SessionId,
	CreatedAt,
	UpdatedAt,
}

#[derive(Iden)]
enum AppQuestions {
	Table,
	Id,
	TestId,
	Question,
	Discussion,
}

#[derive(Iden)]
enum AppOptions {
	Table,
	Id,
	QuestionId,
	Label,
	IsCorrect,
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
enum AppUserAnswers {
	Table,
	Id,
	UserId,
	TestId,
	QuestionId,
	OptionId,
	Answer,
}
