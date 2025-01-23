use axum::response::IntoResponse;

use crate::utils::dto::{MessageResponseDto, MetaRequestDto};

use super::permissions_dto::{
    PermissionsDetailResponseDto, PermissionsListResponseDto, PermissionsRequestDto,
};

#[utoipa::path(
    get,
    path = "/v1/permissions",
    params(MetaRequestDto),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Permissions", body = PermissionsListResponseDto),
        (status = 400, description = "Invalid Permissions data", body = MessageResponseDto)
    ),
    tag = "Permissions"
)]

pub async fn get_permissions() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    get,
    path = "/v1/permissions/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Detail Permission", body = PermissionsDetailResponseDto),
        (status = 400, description = "Invalid Permission data", body = MessageResponseDto)
    ),
    tag = "Permissions"
)]

pub async fn get_detail_permission() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    post,
    path = "/v1/permissions/create",
    request_body = PermissionsRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Permission Created", body = MessageResponseDto),
        (status = 400, description = "Invalid Permission data", body = MessageResponseDto)
    ),
    tag = "Permissions"
)]

pub async fn post_create_permission() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    delete,
    path = "/v1/permissions/delete/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Permission Deleted", body = MessageResponseDto),
        (status = 400, description = "Invalid Permission data", body = MessageResponseDto)
    ),
    tag = "Permissions"
)]

pub async fn delete_permission() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    put,
    path = "/v1/permissions/update/{id}",
    request_body = PermissionsRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Permission Updated", body = MessageResponseDto),
        (status = 400, description = "Invalid Permission data", body = MessageResponseDto)
    ),
    tag = "Permissions"
)]

pub async fn put_update_permission() -> impl IntoResponse {
    ()
}
