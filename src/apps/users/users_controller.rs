use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::utils::meta::TMetaRequest;

use super::{
    users_dto::UsersRequestDto,
    users_repository::{mutation_create_users, query_get_user_by_id, query_get_users},
};

#[utoipa::path(
    get,
    path = "/api/users",
    params(TMetaRequest),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Users", body = UsersListResponseDto),
        (status = 400, description = "Invalid Users data", body = MessageResponse)
    ),
    tag = "Users"
)]

pub async fn get_users(Query(params): Query<TMetaRequest>) -> impl IntoResponse {
    query_get_users(params).await
}

#[utoipa::path(
    get,
    path = "/api/users/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Detail User", body = UsersDetailResponseDto),
        (status = 400, description = "Invalid User data", body = MessageResponse)
    ),
    tag = "Users"
)]

pub async fn get_detail_user(Path(id): Path<Uuid>) -> impl IntoResponse {
    query_get_user_by_id(id).await
}

#[utoipa::path(
    post,
    path = "/api/users/create",
    request_body = UsersRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "User Created", body = MessageResponse),
        (status = 400, description = "Invalid User data", body = MessageResponse)
    ),
    tag = "Users"
)]

pub async fn post_create_user(Json(payload): Json<UsersRequestDto>) -> impl IntoResponse {
    mutation_create_users(Json(payload)).await
}

#[utoipa::path(
    delete,
    path = "/api/users/delete/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "User Deleted", body = MessageResponse),
        (status = 400, description = "Invalid User data", body = MessageResponse)
    ),
    tag = "Users"
)]

pub async fn delete_user() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    put,
    path = "/api/users/update/{id}",
    request_body = UsersRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "User Updated", body = MessageResponse),
        (status = 400, description = "Invalid User data", body = MessageResponse)
    ),
    tag = "Users"
)]

pub async fn put_update_user() -> impl IntoResponse {
    ()
}
