use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// This migration creates the `app_user_question_answers` table.
#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(AppUserQuestionAnswers::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(AppUserQuestionAnswers::Id)
							.uuid()
							.not_null()
							.primary_key(),
					)
					.col(
						ColumnDef::new(AppUserQuestionAnswers::AnswerId)
							.uuid()
							.not_null(),
					)
					.col(
						ColumnDef::new(AppUserQuestionAnswers::QuestionId)
							.uuid()
							.not_null(),
					)
					.col(
						ColumnDef::new(AppUserQuestionAnswers::OptionId)
							.uuid()
							.not_null(),
					)
					// Foreign key: answer_id -> app_user_answers(id)
					.foreign_key(
						ForeignKey::create()
							.name("fk-app_user_question_answers-answer_id")
							.from(
								AppUserQuestionAnswers::Table,
								AppUserQuestionAnswers::AnswerId,
							)
							.to(AppUserAnswers::Table, AppUserAnswers::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					// Foreign key: question_id -> app_questions(id)
					.foreign_key(
						ForeignKey::create()
							.name("fk-app_user_question_answers-question_id")
							.from(
								AppUserQuestionAnswers::Table,
								AppUserQuestionAnswers::QuestionId,
							)
							.to(AppQuestions::Table, AppQuestions::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					// Foreign key: option_id -> app_options(id)
					.foreign_key(
						ForeignKey::create()
							.name("fk-app_user_question_answers-option_id")
							.from(
								AppUserQuestionAnswers::Table,
								AppUserQuestionAnswers::OptionId,
							)
							.to(AppOptions::Table, AppOptions::Id)
							.on_delete(ForeignKeyAction::Cascade)
							.on_update(ForeignKeyAction::Cascade),
					)
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(
				Table::drop()
					.table(AppUserQuestionAnswers::Table)
					.to_owned(),
			)
			.await
	}
}

/// Identifier for the `app_user_question_answers` table and its columns.
#[derive(Iden)]
enum AppUserQuestionAnswers {
	Table,
	Id,
	AnswerId,
	QuestionId,
	OptionId,
}

/// Identifier for the `app_user_answers` table.
/// (Adjust the table name and columns if they differ in your project.)
#[derive(Iden)]
enum AppUserAnswers {
	Table,
	Id,
}

/// Identifier for the `app_questions` table.
/// (Adjust the table name and columns if they differ in your project.)
#[derive(Iden)]
enum AppQuestions {
	Table,
	Id,
}

/// Identifier for the `app_options` table.
/// (Adjust the table name and columns if they differ in your project.)
#[derive(Iden)]
enum AppOptions {
	Table,
	Id,
}
