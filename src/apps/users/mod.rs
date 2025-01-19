use axum::{routing::get, Router};

pub mod users_controller;
pub mod users_dto;
pub mod users_repository;

pub fn users_router() -> Router {
    Router::new().route("/", get(users_controller::get_users))
}
