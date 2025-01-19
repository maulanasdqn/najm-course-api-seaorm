pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_tables;
mod m20250118_201651_add_created_at_updated_at;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_tables::Migration),
            Box::new(m20250118_201651_add_created_at_updated_at::Migration),
        ]
    }
}
