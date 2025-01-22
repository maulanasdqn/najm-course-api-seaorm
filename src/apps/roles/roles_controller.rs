use crate::{apps::auth::auth_dto::MessageResponse, utils::meta::TMetaRequest};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use uuid::Uuid;

use super::{
    roles_dto::{RolesDetailResponseDto, RolesListResponseDto, RolesRequestDto},
    roles_repository::{query_get_role_by_id, query_get_roles},
};

#[utoipa::path(
    get,
    path = "/api/roles",
    params(TMetaRequest),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Roles", body = RolesListResponseDto),
        (status = 400, description = "Invalid Roles data", body = MessageResponse)
    ),
    tag = "Roles"
)]

pub async fn get_roles(Query(params): Query<TMetaRequest>) -> impl IntoResponse {
    query_get_roles(params).await
}

#[utoipa::path(
    get,
    path = "/api/roles/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Detail Role", body = RolesDetailResponseDto),
        (status = 400, description = "Invalid Role data", body = MessageResponse)
    ),
    tag = "Roles"
)]

pub async fn get_detail_role(Path(id): Path<Uuid>) -> impl IntoResponse {
    query_get_role_by_id(id).await
}

#[utoipa::path(
    post,
    path = "/api/roles/create",
    request_body = RolesRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Role Created", body = MessageResponse),
        (status = 400, description = "Invalid Role data", body = MessageResponse)
    ),
    tag = "Roles"
)]

pub async fn post_create_role() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    delete,
    path = "/api/roles/delete/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Role Deleted", body = MessageResponse),
        (status = 400, description = "Invalid Role data", body = MessageResponse)
    ),
    tag = "Roles"
)]

pub async fn delete_role() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    put,
    path = "/api/roles/update/{id}",
    request_body = RolesRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Role Updated", body = MessageResponse),
        (status = 400, description = "Invalid Role data", body = MessageResponse)
    ),
    tag = "Roles"
)]

pub async fn put_update_role() -> impl IntoResponse {
    ()
}
