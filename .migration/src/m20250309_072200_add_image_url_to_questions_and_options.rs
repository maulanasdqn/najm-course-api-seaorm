use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	// Add `image_url` column to both tables.
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		// Alter the app_questions table.
		manager
			.alter_table(
				Table::alter()
					.table(AppQuestions::Table)
					.add_column(
						ColumnDef::new(AppQuestions::ImageUrl).string().null(),
					)
					.to_owned(),
			)
			.await?;

		// Alter the app_options table.
		manager
			.alter_table(
				Table::alter()
					.table(AppOptions::Table)
					.add_column(ColumnDef::new(AppOptions::ImageUrl).string().null())
					.to_owned(),
			)
			.await?;

		Ok(())
	}

	// Remove `image_url` column from both tables.
	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		// Revert change on app_questions table.
		manager
			.alter_table(
				Table::alter()
					.table(AppQuestions::Table)
					.drop_column(AppQuestions::ImageUrl)
					.to_owned(),
			)
			.await?;

		// Revert change on app_options table.
		manager
			.alter_table(
				Table::alter()
					.table(AppOptions::Table)
					.drop_column(AppOptions::ImageUrl)
					.to_owned(),
			)
			.await?;

		Ok(())
	}
}

#[derive(Iden)]
enum AppQuestions {
	Table,
	ImageUrl,
}

#[derive(Iden)]
enum AppOptions {
	Table,
	ImageUrl,
}
