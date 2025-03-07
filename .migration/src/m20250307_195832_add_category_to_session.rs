use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(AppTestSessions::Table)
					.add_column(
						ColumnDef::new(AppTestSessions::Category)
							.string()
							.not_null()
							.default("All"),
					)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(AppTestSessions::Table)
					.drop_column(AppTestSessions::Category)
					.to_owned(),
			)
			.await
	}
}

#[derive(Iden)]
enum AppTestSessions {
	#[iden = "app_test_sessions"]
	Table,
	#[iden = "category"]
	Category,
}
