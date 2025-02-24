use crate::Config;
use log::{error, info};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub mod schemas;
pub mod seeders;

pub use schemas::*;
pub use seeders::*;

pub async fn get_db() -> DatabaseConnection {
	let config = Config::new();
	let database_url = config.database_url;
	let mut opt = ConnectOptions::new(&database_url);
	opt.max_connections(100)
		.min_connections(5)
		.connect_timeout(Duration::from_secs(8))
		.acquire_timeout(Duration::from_secs(8))
		.idle_timeout(Duration::from_secs(8))
		.max_lifetime(Duration::from_secs(8))
		.sqlx_logging(true)
		.sqlx_logging_level(log::LevelFilter::Info)
		.set_schema_search_path("public");

	match Database::connect(opt).await {
		Ok(db_connection) => {
			info!("Successfully connected to the database.");
			db_connection
		}
		Err(err) => {
			error!("Failed to connect to the database: {}", err);
			panic!("Database connection failed: {}", err);
		}
	}
}
