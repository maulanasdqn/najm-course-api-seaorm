use crate::utils::meta::TMetaRequest;
use axum::response::IntoResponse;

#[utoipa::path(
    get,
    path = "/api/permissions",
    params(TMetaRequest),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Permissions", body = PermissionsListResponseDto),
        (status = 400, description = "Invalid Permissions data", body = MessageResponse)
    ),
    tag = "Permissions"
)]

pub async fn get_permissions() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    get,
    path = "/api/permissions/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Detail Permission", body = PermissionsDetailResponseDto),
        (status = 400, description = "Invalid Permission data", body = MessageResponse)
    ),
    tag = "Permissions"
)]

pub async fn get_detail_permission() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    post,
    path = "/api/permissions/create",
    request_body = PermissionsRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Permission Created", body = MessageResponse),
        (status = 400, description = "Invalid Permission data", body = MessageResponse)
    ),
    tag = "Permissions"
)]

pub async fn post_create_permission() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    delete,
    path = "/api/permissions/delete/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Permission Deleted", body = MessageResponse),
        (status = 400, description = "Invalid Permission data", body = MessageResponse)
    ),
    tag = "Permissions"
)]

pub async fn delete_permission() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    put,
    path = "/api/permissions/update/{id}",
    request_body = PermissionsRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Permission Updated", body = MessageResponse),
        (status = 400, description = "Invalid Permission data", body = MessageResponse)
    ),
    tag = "Permissions"
)]

pub async fn put_update_permission() -> impl IntoResponse {
    ()
}
