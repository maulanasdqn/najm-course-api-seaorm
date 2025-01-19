use crate::utils::meta::TMetaRequest;
use axum::response::IntoResponse;

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

pub async fn get_roles() -> impl IntoResponse {
    ()
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

pub async fn get_detail_role() -> impl IntoResponse {
    ()
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
