use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
					.foreign_key(
						ForeignKey::create()
							.name("fk_app_user_answers_user_id")
							.from(AppUserAnswers::Table, AppUserAnswers::UserId)
							.to(AppUsers::Table, AppUsers::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.foreign_key(
						ForeignKey::create()
							.name("fk_app_user_answers_test_id")
							.from(AppUserAnswers::Table, AppUserAnswers::TestId)
							.to(AppTests::Table, AppTests::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(AppUserAnswers::Table).to_owned())
			.await
	}
}

#[derive(Iden)]
enum AppUserAnswers {
	#[iden = "app_user_answers"]
	Table,
	#[iden = "id"]
	Id,
	#[iden = "user_id"]
	UserId,
	#[iden = "test_id"]
	TestId,
}

#[derive(Iden)]
enum AppUsers {
	#[iden = "app_users"]
	Table,
	#[iden = "id"]
	Id,
}

#[derive(Iden)]
enum AppTests {
	#[iden = "app_tests"]
	Table,
	#[iden = "id"]
	Id,
}
