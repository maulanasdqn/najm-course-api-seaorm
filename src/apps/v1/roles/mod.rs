use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub mod roles_controller;
pub mod roles_dto;
pub mod roles_enum;
pub mod roles_repository;

pub use roles_dto::*;
pub use roles_enum::*;
pub use roles_repository::*;

pub fn roles_router() -> Router {
    Router::new()
        .route("/", get(roles_controller::get_roles))
        .route("/create", post(roles_controller::post_create_role))
        .route("/detail/{id}", get(roles_controller::get_detail_role))
        .route("/update/{id}", put(roles_controller::put_update_role))
        .route("/delete/{id}", delete(roles_controller::delete_role))
}
