use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::utils::dto::{MessageResponseDto, MetaRequestDto};

use super::{
    roles_dto::{RolesDetailResponseDto, RolesListResponseDto, RolesRequestDto},
    roles_repository::{mutation_create_role, query_get_role_by_id, query_get_roles},
};

#[utoipa::path(
    get,
    path = "/v1/roles",
    params(MetaRequestDto),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Roles", body = RolesListResponseDto),
        (status = 400, description = "Invalid Roles data", body = MessageResponseDto)
    ),
    tag = "Roles"
)]

pub async fn get_roles(Query(params): Query<MetaRequestDto>) -> impl IntoResponse {
    query_get_roles(params).await
}

#[utoipa::path(
    get,
    path = "/v1/roles/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Detail Role", body = RolesDetailResponseDto),
        (status = 400, description = "Invalid Role data", body = MessageResponseDto)
    ),
    tag = "Roles"
)]

pub async fn get_detail_role(Path(id): Path<Uuid>) -> impl IntoResponse {
    query_get_role_by_id(id).await
}

#[utoipa::path(
    post,
    path = "/v1/roles/create",
    request_body = RolesRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Role Created", body = MessageResponseDto),
        (status = 400, description = "Invalid Role data", body = MessageResponseDto)
    ),
    tag = "Roles"
)]

pub async fn post_create_role(Json(payload): Json<RolesRequestDto>) -> impl IntoResponse {
    mutation_create_role(Json(payload)).await
}

#[utoipa::path(
    delete,
    path = "/v1/roles/delete/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Role Deleted", body = MessageResponseDto),
        (status = 400, description = "Invalid Role data", body = MessageResponseDto)
    ),
    tag = "Roles"
)]

pub async fn delete_role() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    put,
    path = "/v1/roles/update/{id}",
    request_body = RolesRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Role Updated", body = MessageResponseDto),
        (status = 400, description = "Invalid Role data", body = MessageResponseDto)
    ),
    tag = "Roles"
)]

pub async fn put_update_role() -> impl IntoResponse {
    ()
}
