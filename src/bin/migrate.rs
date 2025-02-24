use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::env;

#[tokio::main]
async fn main() {
	let database_url = env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set in the environment or .env file");

	let db = Database::connect(&database_url)
		.await
		.expect("Failed to connect to the database");

	println!("Running migrations...");

	Migrator::up(&db, None).await.expect("Migration failed");

	println!("Migrations applied successfully.");
}
