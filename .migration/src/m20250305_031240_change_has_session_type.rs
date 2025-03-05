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
					.modify_column(
						ColumnDef::new(AppSessionsHasTests::Weight).string().null(),
					)
					.modify_column(
						ColumnDef::new(AppSessionsHasTests::Multiplier)
							.string()
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
					.modify_column(
						ColumnDef::new(AppSessionsHasTests::Weight).float().null(),
					)
					.modify_column(
						ColumnDef::new(AppSessionsHasTests::Multiplier)
							.float()
							.null(),
					)
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
