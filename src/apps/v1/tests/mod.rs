use axum::{
	routing::{delete, get, post, put},
	Router,
};

pub mod tests_controller;
pub mod tests_dto;
pub mod tests_repository;

pub use tests_dto::*;
pub use tests_repository::*;

pub fn tests_router() -> Router {
	Router::new()
		.route("/", get(tests_controller::get_tests))
		.route("/create", post(tests_controller::post_create_test))
		.route("/detail/{id}", get(tests_controller::get_detail_test))
		.route("/update/{id}", put(tests_controller::put_update_test))
		.route("/delete/{id}", delete(tests_controller::delete_test))
}
