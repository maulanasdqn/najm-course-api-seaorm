pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250227_001403_add_sessions_student_type;
mod m20250228_214441_adjust_sessions_table;
mod m20250302_082406_adjust_answers;
mod m20250303_155045_adjust_test_and_sessions;
mod m20250304_160308_add_weight_and_multiplier;
mod m20250304_163253_remove_test_session_id;
mod m20250305_031240_change_has_session_type;
mod m20250307_195832_add_category_to_session;
mod m20250308_154930_user_question_answer;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
			Box::new(m20220101_000001_create_table::Migration),
			Box::new(m20250227_001403_add_sessions_student_type::Migration),
			Box::new(m20250228_214441_adjust_sessions_table::Migration),
			Box::new(m20250302_082406_adjust_answers::Migration),
			Box::new(m20250303_155045_adjust_test_and_sessions::Migration),
			Box::new(m20250304_160308_add_weight_and_multiplier::Migration),
			Box::new(m20250304_163253_remove_test_session_id::Migration),
			Box::new(m20250305_031240_change_has_session_type::Migration),
			Box::new(m20250307_195832_add_category_to_session::Migration),
			Box::new(m20250308_154930_user_question_answer::Migration),
		]
	}
}
