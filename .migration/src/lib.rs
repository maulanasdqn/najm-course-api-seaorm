pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250227_001403_add_sessions_student_type;
mod m20250228_214441_adjust_sessions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250227_001403_add_sessions_student_type::Migration),
            Box::new(m20250228_214441_adjust_sessions_table::Migration),
        ]
    }
}
