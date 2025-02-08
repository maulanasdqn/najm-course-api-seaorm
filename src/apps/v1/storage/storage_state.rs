use crate::{AppState, MinioClient};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn storage_state() -> Result<AppState, Box<dyn std::error::Error>> {
	let minio_client = MinioClient::new().await?;
	Ok(AppState {
		minio: Arc::new(Mutex::new(minio_client)),
	})
}
