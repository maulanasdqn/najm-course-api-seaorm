use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    response::IntoResponse,
    Json,
};

use crate::{
    permissions::{permissions_middleware, PermissionsEnum},
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
pub async fn get_permissions(
    headers: HeaderMap,
    Query(params): Query<MetaRequestDto>,
) -> impl IntoResponse {
    match permissions_middleware(headers, vec![PermissionsEnum::ReadListPermissions]).await {
        Ok(_) => query_get_permissions(params).await,
        Err(response) => response,
    }
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
pub async fn get_detail_permission(
    headers: HeaderMap,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match permissions_middleware(headers, vec![PermissionsEnum::ReadDetailPermissions]).await {
        Ok(_) => query_get_permission_by_id(id).await,
        Err(response) => response,
    }
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
    headers: HeaderMap,
    Json(payload): Json<PermissionsRequestDto>,
) -> impl IntoResponse {
    match permissions_middleware(headers, vec![PermissionsEnum::CreatePermissions]).await {
        Ok(_) => mutation_create_permission(Json(payload)).await,
        Err(response) => response,
    }
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
pub async fn delete_permission(headers: HeaderMap, Path(id): Path<String>) -> impl IntoResponse {
    match permissions_middleware(headers, vec![PermissionsEnum::DeletePermissions]).await {
        Ok(_) => mutation_delete_permission(id).await,
        Err(response) => response,
    }
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
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<PermissionsRequestDto>,
) -> impl IntoResponse {
    match permissions_middleware(headers, vec![PermissionsEnum::UpdatePermissions]).await {
        Ok(_) => mutation_update_permission(id, Json(payload)).await,
        Err(response) => response,
    }
}
