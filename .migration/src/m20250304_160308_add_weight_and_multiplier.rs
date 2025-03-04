use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(AppSessionsHasTests::Table)
					.add_column(
						ColumnDef::new(AppSessionsHasTests::Weight).float().null(),
					)
					.add_column(
						ColumnDef::new(AppSessionsHasTests::Multiplier)
							.float()
							.null(),
					)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(AppSessionsHasTests::Table)
					.drop_column(AppSessionsHasTests::Weight)
					.drop_column(AppSessionsHasTests::Multiplier)
					.to_owned(),
			)
			.await
	}
}

#[derive(Iden)]
enum AppSessionsHasTests {
	#[iden = "app_sessions_has_tests"]
	Table,
	#[iden = "weight"]
	Weight,
	#[iden = "multiplier"]
	Multiplier,
}
