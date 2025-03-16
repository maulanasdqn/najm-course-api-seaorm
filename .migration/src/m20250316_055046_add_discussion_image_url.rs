use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(AppQuestions::Table)
					.add_column(
						ColumnDef::new(AppQuestions::DiscussionImageUrl)
							.string()
							.null(),
					)
					.to_owned(),
			)
			.await?;

		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				Table::alter()
					.table(AppQuestions::Table)
					.drop_column(AppQuestions::DiscussionImageUrl)
					.to_owned(),
			)
			.await?;

		Ok(())
	}
}

#[derive(Iden)]
enum AppQuestions {
	Table,
	DiscussionImageUrl,
}
