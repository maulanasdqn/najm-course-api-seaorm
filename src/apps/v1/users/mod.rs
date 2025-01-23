use axum::{
    routing::{get, post},
    Router,
};

pub mod users_controller;
pub mod users_dto;
pub mod users_repository;

pub fn users_router() -> Router {
    Router::new()
        .route("/", get(users_controller::get_users))
        .route("/create", post(users_controller::post_create_user))
        .route("/detail/{id}", get(users_controller::get_detail_user))
        .route("/update/{id}", post(users_controller::put_update_user))
        .route("/delete/{id}", post(users_controller::delete_user))
}
