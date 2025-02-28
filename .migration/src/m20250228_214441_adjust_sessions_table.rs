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
						ColumnDef::new(AppTestSessions::IsActive)
							.boolean()
							.not_null()
							.default(false),
					)
					.add_column(
						ColumnDef::new(AppTestSessions::Description)
							.string()
							.not_null()
							.default(""),
					)
					.drop_column(AppTestSessions::StartDate)
					.drop_column(AppTestSessions::EndDate)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(AppTestSessions::Table)
					.drop_column(AppTestSessions::IsActive)
					.drop_column(AppTestSessions::Description)
					.add_column(ColumnDef::new(AppTestSessions::StartDate))
					.add_column(ColumnDef::new(AppTestSessions::EndDate))
					.to_owned(),
			)
			.await
	}
}

#[derive(DeriveIden)]
pub enum AppTestSessions {
	Table,
	IsActive,
	StartDate,
	EndDate,
	Description,
}
