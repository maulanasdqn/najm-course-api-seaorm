use axum::{routing::post, Router};

pub mod auth_controller;
pub mod auth_dto;
pub mod auth_middleware;
pub mod auth_repository;

pub use auth_dto::*;
pub use auth_middleware::*;
pub use auth_repository::*;

pub fn auth_router() -> Router {
    Router::new()
        .route("/login", post(auth_controller::post_login))
        .route("/register", post(auth_controller::post_register))
        .route("/forgot", post(auth_controller::post_forgot))
        .route("/verify-email", post(auth_controller::post_verify_email))
        .route("/send-otp", post(auth_controller::post_send_otp))
        .route("/new-password", post(auth_controller::post_new_password))
        .route("/refresh", post(auth_controller::post_refresh))
}
