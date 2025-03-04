use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(AppTests::Table)
					.drop_column(AppTests::SessionId)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(AppTests::Table)
					.add_column(
						ColumnDef::new(AppTests::SessionId).uuid().not_null(),
					)
					.to_owned(),
			)
			.await
	}
}

#[derive(Iden)]
enum AppTests {
	#[iden = "app_tests"]
	Table,
	#[iden = "session_id"]
	SessionId,
}
