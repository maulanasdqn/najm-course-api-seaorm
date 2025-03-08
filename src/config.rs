use std::env;

pub struct Config {
	pub port: String,
	pub access_token_secret: String,
	pub refresh_token_secret: String,
	pub database_url: String,
	pub database_schema: String,
	pub smtp_email: String,
	pub smtp_password: String,
	pub smtp_name: String,
	pub redis_hostname: String,
	pub fe_url: String,
	pub rust_env: String,
	pub minio_endpoint: String,
	pub minio_bucket_name: String,
	pub minio_access_key: String,
	pub minio_secret_key: String,
}

impl Config {
	pub fn new() -> Self {
		Self {
			port: env::var("PORT").unwrap_or("3000".to_string()),
			access_token_secret: env::var("ACCESS_TOKEN_SECRET")
				.unwrap_or("default_access_secret".to_string()),
			refresh_token_secret: env::var("REFRESH_TOKEN_SECRET")
				.unwrap_or("default_refresh_secret".to_string()),
			database_url: env::var("DATABASE_URL")
				.unwrap_or("postgres://localhost".to_string()),
			database_schema: env::var("DATABASE_SCHEMA")
				.unwrap_or("public".to_string()),
			smtp_email: env::var("SMTP_EMAIL")
				.unwrap_or("no-reply@example.com".to_string()),
			smtp_password: env::var("SMTP_PASSWORD")
				.unwrap_or("default_smtp_password".to_string()),
			smtp_name: env::var("SMTP_NAME").unwrap_or("MyApp SMTP".to_string()),
			redis_hostname: env::var("REDIS_HOSTNAME")
				.unwrap_or("localhost".to_string()),
			fe_url: env::var("FE_URL").unwrap_or("http://localhost".to_string()),
			rust_env: env::var("RUST_ENV").unwrap_or("development".to_string()),
			minio_endpoint: env::var("MINIO_ENDPOINT")
				.unwrap_or("http://localhost:9000".to_string()),
			minio_bucket_name: env::var("MINIO_BUCKET_NAME")
				.unwrap_or("default_bucket".to_string()),
			minio_access_key: env::var("MINIO_ACCESS_KEY")
				.unwrap_or("minio_access".to_string()),
			minio_secret_key: env::var("MINIO_SECRET_KEY")
				.unwrap_or("minio_secret".to_string()),
		}
	}

	pub fn format(&self) -> String {
		format!(
            "Port: {}\nAccess Token Secret: {}\nRefresh Token Secret: {}\nDatabase URL: {}\nDatabase Schema: {}\nSMTP Email: {}\nSMTP Password: {}\nSMTP Name: {}\nRedis Hostname: {}\nFE URL: {}\nRust Env: {}\nMinio Endpoint: {}\nMinio Bucket Name: {}\nMinio Access Key: {}\nMinio Secret Key: {}",
            self.port,
            self.access_token_secret,
            self.refresh_token_secret,
            self.database_url,
			self.database_schema,
            self.smtp_email,
            self.smtp_password,
            self.smtp_name,
            self.redis_hostname,
            self.fe_url,
            self.rust_env,
            self.minio_endpoint,
            self.minio_bucket_name,
            self.minio_access_key,
            self.minio_secret_key,
        )
	}
}
