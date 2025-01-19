use axum::{extract::Json, response::IntoResponse};

use crate::apps::users::{users_dto::UsersCreateDto, users_repository::mutation_create_users};

use super::{auth_dto::AuthLoginDto, auth_repository::mutation_login};

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = AuthLoginDto,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Unauthorized", body = MessageResponse)
    ),
    tag = "Authentication"
)]

pub async fn post_login(Json(payload): Json<AuthLoginDto>) -> impl IntoResponse {
    mutation_login(Json(payload)).await
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = UsersCreateDto,
    responses(
        (status = 200, description = "Register successful", body = MessageResponse),
        (status = 401, description = "Unauthorized", body = MessageResponse)
    ),
    tag = "Authentication"
)]

pub async fn post_register(Json(payload): Json<UsersCreateDto>) -> impl IntoResponse {
    mutation_create_users(Json(payload)).await
}
