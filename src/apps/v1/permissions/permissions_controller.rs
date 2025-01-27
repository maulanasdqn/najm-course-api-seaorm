use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};

use crate::{
    utils::dto::{MessageResponseDto, MetaRequestDto},
    ResponseSuccessDto, ResponseSuccessListDto,
};

use super::{
    mutation_create_permission, mutation_delete_permission, mutation_update_permission,
    query_get_permission_by_id, query_get_permissions, PermissionsItemDto, PermissionsRequestDto,
};

#[utoipa::path(
    get,
    path = "/v1/permissions",
    params(MetaRequestDto),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Permissions", body = ResponseSuccessListDto<PermissionsItemDto>),
        (status = 400, description = "Invalid Permissions data", body = MessageResponseDto)
    ),
    tag = "Permissions"
)]

pub async fn get_permissions(Query(params): Query<MetaRequestDto>) -> impl IntoResponse {
    query_get_permissions(params).await
}

#[utoipa::path(
    get,
    path = "/v1/permissions/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Detail Permission", body = ResponseSuccessDto<PermissionsItemDto>),
        (status = 400, description = "Invalid Permission data", body = MessageResponseDto)
    ),
    tag = "Permissions"
)]

pub async fn get_detail_permission(Path(id): Path<String>) -> impl IntoResponse {
    query_get_permission_by_id(id).await
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

pub async fn post_create_permission(
    Json(payload): Json<PermissionsRequestDto>,
) -> impl IntoResponse {
    mutation_create_permission(Json(payload)).await
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

pub async fn delete_permission(Path(id): Path<String>) -> impl IntoResponse {
    mutation_delete_permission(id).await
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

pub async fn put_update_permission(
    Path(id): Path<String>,
    Json(payload): Json<PermissionsRequestDto>,
) -> impl IntoResponse {
    mutation_update_permission(id, Json(payload)).await
}
