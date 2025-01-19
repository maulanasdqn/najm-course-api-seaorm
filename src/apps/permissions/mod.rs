use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub mod permissions_controller;
pub mod permissions_dto;
pub mod permissions_repository;

pub fn permissions_router() -> Router {
    Router::new()
        .route("/", get(permissions_controller::get_permissions))
        .route(
            "/detail/:id",
            get(permissions_controller::get_detail_permission),
        )
        .route(
            "/create",
            post(permissions_controller::post_create_permission),
        )
        .route(
            "/update/:id",
            put(permissions_controller::put_update_permission),
        )
        .route(
            "/delete/:id",
            delete(permissions_controller::delete_permission),
        )
}
