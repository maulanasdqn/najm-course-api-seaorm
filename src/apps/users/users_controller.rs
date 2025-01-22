use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{apps::auth::auth_dto::MessageResponse, utils::meta::TMetaRequest};

use super::{
    users_dto::{
        UsersCreateRequestDto, UsersDetailResponseDto, UsersListResponseDto, UsersUpdateRequestDto,
    },
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

pub async fn get_detail_user(Path(id): Path<String>) -> impl IntoResponse {
    match Uuid::parse_str(&id) {
        Ok(uuid) => match query_get_user_by_id(uuid).await {
            Ok(response) => response.into_response(),
            Err(err) => err.into_response(),
        },
        Err(_) => (StatusCode::BAD_REQUEST, "Invalid UUID format").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/users/create",
    request_body = UsersCreateRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "User Created", body = MessageResponse),
        (status = 400, description = "Invalid User data", body = MessageResponse)
    ),
    tag = "Users"
)]

pub async fn post_create_user(Json(payload): Json<UsersCreateRequestDto>) -> impl IntoResponse {
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
    request_body = UsersUpdateRequestDto,
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
