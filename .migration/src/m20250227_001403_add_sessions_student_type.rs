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
						ColumnDef::new(AppTestSessions::StudentType).string().null(),
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
					.drop_column(AppTestSessions::StudentType)
					.to_owned(),
			)
			.await
	}
}

#[derive(DeriveIden)]
pub enum AppTestSessions {
	Table,
	StudentType,
}
