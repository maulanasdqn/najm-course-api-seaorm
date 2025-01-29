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
    mutation_delete_role, mutation_update_role,
    roles_dto::RolesRequestUpdateDto,
    roles_repository::{mutation_create_role, query_get_role_by_id, query_get_roles},
    RolesItemDto, RolesItemListDto, RolesRequestCreateDto,
};

#[utoipa::path(
    get,
    path = "/v1/roles",
    params(MetaRequestDto),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Roles", body = ResponseSuccessListDto<RolesItemListDto>),
        (status = 400, description = "Invalid Roles data", body = MessageResponseDto)
    ),
    tag = "Roles"
)]
pub async fn get_roles(
    headers: HeaderMap,
    Query(params): Query<MetaRequestDto>,
) -> impl IntoResponse {
    match permissions_middleware(&headers, vec![PermissionsEnum::ReadListRoles]).await {
        Ok(_) => query_get_roles(params).await,
        Err(response) => response,
    }
}

#[utoipa::path(
    get,
    path = "/v1/roles/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Detail Role", body = ResponseSuccessDto<RolesItemDto>),
        (status = 400, description = "Invalid Role data", body = MessageResponseDto)
    ),
    tag = "Roles"
)]
pub async fn get_detail_role(headers: HeaderMap, Path(id): Path<String>) -> impl IntoResponse {
    match permissions_middleware(&headers, vec![PermissionsEnum::ReadDetailRoles]).await {
        Ok(_) => query_get_role_by_id(id).await,
        Err(response) => response,
    }
}

#[utoipa::path(
    post,
    path = "/v1/roles/create",
    request_body = RolesRequestCreateDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Role Created", body = MessageResponseDto),
        (status = 400, description = "Invalid Role data", body = MessageResponseDto)
    ),
    tag = "Roles"
)]
pub async fn post_create_role(
    headers: HeaderMap,
    Json(payload): Json<RolesRequestCreateDto>,
) -> impl IntoResponse {
    match permissions_middleware(&headers, vec![PermissionsEnum::CreateRoles]).await {
        Ok(_) => mutation_create_role(Json(payload)).await,
        Err(response) => response,
    }
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
pub async fn delete_role(headers: HeaderMap, Path(id): Path<String>) -> impl IntoResponse {
    match permissions_middleware(&headers, vec![PermissionsEnum::DeleteRoles]).await {
        Ok(_) => mutation_delete_role(id).await,
        Err(response) => response,
    }
}

#[utoipa::path(
    put,
    path = "/v1/roles/update/{id}",
    request_body = RolesRequestUpdateDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Role Updated", body = MessageResponseDto),
        (status = 400, description = "Invalid Role data", body = MessageResponseDto)
    ),
    tag = "Roles"
)]
pub async fn put_update_role(
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<RolesRequestUpdateDto>,
) -> impl IntoResponse {
    match permissions_middleware(&headers, vec![PermissionsEnum::UpdateRoles]).await {
        Ok(_) => mutation_update_role(id, Json(payload)).await,
        Err(response) => response,
    }
}
