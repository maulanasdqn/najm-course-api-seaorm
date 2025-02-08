pub mod storage_controller;
pub mod storage_dto;
pub mod storage_repository;
pub mod storage_state;

use crate::storage_state::storage_state;
use axum::{routing::post, Router};

pub use storage_controller::*;
pub use storage_dto::*;
pub use storage_repository::*;

pub async fn storage_router() -> Router {
	let state = storage_state()
		.await
		.expect("Failed to initialize storage state");

	Router::new().route(
		"/upload",
		post(storage_controller::post_upload).with_state(state),
	)
}
