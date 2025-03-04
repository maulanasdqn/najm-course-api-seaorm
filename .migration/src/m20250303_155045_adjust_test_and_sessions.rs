use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(AppSessionsHasTests::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(AppSessionsHasTests::Id)
							.uuid()
							.not_null()
							.primary_key(),
					)
					.col(
						ColumnDef::new(AppSessionsHasTests::SessionId)
							.uuid()
							.not_null(),
					)
					.col(
						ColumnDef::new(AppSessionsHasTests::TestId)
							.uuid()
							.not_null(),
					)
					.col(
						ColumnDef::new(AppSessionsHasTests::StartDate)
							.timestamp_with_time_zone()
							.null(),
					)
					.col(
						ColumnDef::new(AppSessionsHasTests::EndDate)
							.timestamp_with_time_zone()
							.null(),
					)
					.foreign_key(
						ForeignKey::create()
							.from(
								AppSessionsHasTests::Table,
								AppSessionsHasTests::SessionId,
							)
							.to(AppTestSessions::Table, AppTestSessions::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.foreign_key(
						ForeignKey::create()
							.from(
								AppSessionsHasTests::Table,
								AppSessionsHasTests::TestId,
							)
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
			.drop_table(Table::drop().table(AppSessionsHasTests::Table).to_owned())
			.await
	}
}

#[derive(Iden)]
enum AppSessionsHasTests {
	#[iden = "app_sessions_has_tests"]
	Table,
	#[iden = "id"]
	Id,
	#[iden = "session_id"]
	SessionId,
	#[iden = "test_id"]
	TestId,
	#[iden = "start_date"]
	StartDate,
	#[iden = "end_date"]
	EndDate,
}

#[derive(Iden)]
enum AppTestSessions {
	#[iden = "app_test_sessions"]
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
