use axum::{
	routing::{delete, get, post, put},
	Router,
};

pub mod permissions_controller;
pub mod permissions_dto;
pub mod permissions_enum;
pub mod permissions_middleware;
pub mod permissions_repository;

pub use permissions_dto::*;
pub use permissions_enum::*;
pub use permissions_middleware::*;
pub use permissions_repository::*;

pub fn permissions_router() -> Router {
	Router::new()
		.route("/", get(permissions_controller::get_permissions))
		.route(
			"/create",
			post(permissions_controller::post_create_permission),
		)
		.route(
			"/detail/{id}",
			get(permissions_controller::get_detail_permission),
		)
		.route(
			"/update/{id}",
			put(permissions_controller::put_update_permission),
		)
		.route(
			"/delete/{id}",
			delete(permissions_controller::delete_permission),
		)
}
