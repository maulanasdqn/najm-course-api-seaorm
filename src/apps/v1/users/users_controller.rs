use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};

use crate::{MessageResponseDto, MetaRequestDto};

use super::{
    users_dto::{
        UsersCreateRequestDto, UsersDetailResponseDto, UsersListResponseDto, UsersUpdateRequestDto,
    },
    users_repository::{mutation_create_users, query_get_user_by_id, query_get_users},
};

#[utoipa::path(
    get,
    path = "/v1/users",
    params(MetaRequestDto),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Users", body = UsersListResponseDto),
        (status = 400, description = "Invalid Users data", body = MessageResponseDto)
    ),
    tag = "Users"
)]

pub async fn get_users(Query(params): Query<MetaRequestDto>) -> impl IntoResponse {
    query_get_users(params).await
}

#[utoipa::path(
    get,
    path = "/v1/users/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Detail User", body = UsersDetailResponseDto),
        (status = 400, description = "Invalid User data", body = MessageResponseDto)
    ),
    tag = "Users"
)]

pub async fn get_detail_user(Path(id): Path<String>) -> impl IntoResponse {
    query_get_user_by_id(id).await
}

#[utoipa::path(
    post,
    path = "/v1/users/create",
    request_body = UsersCreateRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "User Created", body = MessageResponseDto),
        (status = 400, description = "Invalid User data", body = MessageResponseDto)
    ),
    tag = "Users"
)]

pub async fn post_create_user(Json(payload): Json<UsersCreateRequestDto>) -> impl IntoResponse {
    mutation_create_users(Json(payload)).await
}

#[utoipa::path(
    delete,
    path = "/v1/users/delete/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "User Deleted", body = MessageResponseDto),
        (status = 400, description = "Invalid User data", body = MessageResponseDto)
    ),
    tag = "Users"
)]

pub async fn delete_user() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    put,
    path = "/v1/users/update/{id}",
    request_body = UsersUpdateRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "User Updated", body = MessageResponseDto),
        (status = 400, description = "Invalid User data", body = MessageResponseDto)
    ),
    tag = "Users"
)]

pub async fn put_update_user() -> impl IntoResponse {
    ()
}
