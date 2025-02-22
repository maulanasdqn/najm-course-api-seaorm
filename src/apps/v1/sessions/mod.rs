use axum::{
	routing::{delete, get, post, put},
	Router,
};

pub mod sessions_controller;
pub mod sessions_dto;
pub mod sessions_repository;

pub use sessions_dto::*;
pub use sessions_repository::*;

pub fn sessions_router() -> Router {
	Router::new()
		.route("/", get(sessions_controller::get_sessions))
		.route("/create", post(sessions_controller::post_create_session))
		.route("/detail/{id}", get(sessions_controller::get_detail_session))
		.route("/update/{id}", put(sessions_controller::put_update_session))
		.route("/delete/{id}", delete(sessions_controller::delete_session))
}
