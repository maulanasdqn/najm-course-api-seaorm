use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use hyper::HeaderMap;

use crate::{MessageResponseDto, MetaRequestDto, ResponseSuccessDto, ResponseSuccessListDto};

use super::{
    mutation_delete_user, mutation_update_user, query_get_user_me,
    users_dto::{UsersCreateRequestDto, UsersUpdateRequestDto},
    users_repository::{mutation_create_users, query_get_user_by_id, query_get_users},
    UsersItemDto, UsersItemListDto,
};

#[utoipa::path(
    get,
    path = "/v1/users",
    params(MetaRequestDto),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Users", body = ResponseSuccessListDto<UsersItemListDto>),
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
        (status = 201, description = "Detail User", body = ResponseSuccessDto<UsersItemDto>),
        (status = 400, description = "Invalid User data", body = MessageResponseDto)
    ),
    tag = "Users"
)]

pub async fn get_detail_user(Path(id): Path<String>) -> impl IntoResponse {
    query_get_user_by_id(id).await
}

#[utoipa::path(
    get,
    path = "/v1/users/me",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Detail User Me", body = ResponseSuccessDto<UsersItemDto>),
        (status = 400, description = "Invalid User data", body = MessageResponseDto)
    ),
    tag = "Users"
)]

pub async fn get_user_me(header: HeaderMap) -> impl IntoResponse {
    query_get_user_me(header).await
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

pub async fn delete_user(Path(id): Path<String>) -> impl IntoResponse {
    mutation_delete_user(id).await
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

pub async fn put_update_user(
    Path(id): Path<String>,
    Json(payload): Json<UsersUpdateRequestDto>,
) -> impl IntoResponse {
    mutation_update_user(id, Json(payload)).await
}
