use axum::{
	routing::{delete, get, post, put},
	Router,
};

pub mod users_controller;
pub mod users_dto;
pub mod users_repository;

pub use users_dto::*;
pub use users_repository::*;

use super::auth_controller;

pub fn users_router() -> Router {
	Router::new()
		.route("/", get(users_controller::get_users))
		.route("/me", get(users_controller::get_user_me))
		.route("/create", post(users_controller::post_create_user))
		.route(
			"/change-password",
			post(auth_controller::post_change_password),
		)
		.route("/detail/{id}", get(users_controller::get_detail_user))
		.route("/update/{id}", put(users_controller::put_update_user))
		.route("/update/me", put(users_controller::put_update_user_me))
		.route("/activate/{id}", put(users_controller::put_activate_user))
		.route("/delete/{id}", delete(users_controller::delete_user))
}
