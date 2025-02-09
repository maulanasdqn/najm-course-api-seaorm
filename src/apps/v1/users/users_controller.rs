use axum::{
	extract::{Path, Query},
	response::IntoResponse,
	Json,
};
use hyper::HeaderMap;

use crate::{
	permissions::{permissions_middleware, PermissionsEnum},
	MessageResponseDto, MetaRequestDto, ResponseSuccessDto, ResponseSuccessListDto,
};

use super::{
	mutation_delete_user, mutation_set_active_inactive_user, mutation_update_user,
	mutation_update_user_me, query_get_user_me,
	users_dto::{UsersCreateRequestDto, UsersUpdateRequestDto},
	users_repository::{
		mutation_create_users, query_get_user_by_id, query_get_users,
	},
	UsersActiveInactiveRequestDto, UsersItemDto, UsersItemListDto,
};

#[utoipa::path(
    get,
    path = "/v1/users",
    params(MetaRequestDto),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Users", body = ResponseSuccessListDto<UsersItemListDto>),
        (status = 400, description = "Invalid Users data", body = MessageResponseDto)
    ),
    tag = "Users"
)]
pub async fn get_users(
	headers: HeaderMap,
	Query(params): Query<MetaRequestDto>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::ReadListUsers])
		.await
	{
		Ok(_) => query_get_users(params).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    get,
    path = "/v1/users/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Detail User", body = ResponseSuccessDto<UsersItemDto>),
        (status = 400, description = "Invalid User data", body = MessageResponseDto)
    ),
    tag = "Users"
)]
pub async fn get_detail_user(
	headers: HeaderMap,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::ReadDetailUsers])
		.await
	{
		Ok(_) => query_get_user_by_id(id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    get,
    path = "/v1/users/me",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Detail User Me", body = ResponseSuccessDto<UsersItemDto>),
        (status = 400, description = "Invalid User data", body = MessageResponseDto)
    ),
    tag = "Users"
)]
pub async fn get_user_me(headers: HeaderMap) -> impl IntoResponse {
	query_get_user_me(headers).await
}

#[utoipa::path(
    post,
    path = "/v1/users/create",
    request_body = UsersCreateRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "User Created", body = MessageResponseDto),
        (status = 400, description = "Invalid User data", body = MessageResponseDto)
    ),
    tag = "Users"
)]
pub async fn post_create_user(
	headers: HeaderMap,
	Json(payload): Json<UsersCreateRequestDto>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::CreateUsers]).await
	{
		Ok(_) => mutation_create_users(Json(payload)).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    delete,
    path = "/v1/users/delete/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "User Deleted", body = MessageResponseDto),
        (status = 400, description = "Invalid User data", body = MessageResponseDto)
    ),
    tag = "Users"
)]
pub async fn delete_user(
	headers: HeaderMap,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::DeleteUsers]).await
	{
		Ok(_) => mutation_delete_user(id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    put,
    path = "/v1/users/update/{id}",
    request_body = UsersUpdateRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "User Updated", body = MessageResponseDto),
        (status = 400, description = "Invalid User data", body = MessageResponseDto)
    ),
    tag = "Users"
)]
pub async fn put_update_user(
	headers: HeaderMap,
	Path(id): Path<String>,
	Json(payload): Json<UsersUpdateRequestDto>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::UpdateUsers]).await
	{
		Ok(_) => mutation_update_user(id, Json(payload)).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    put,
    path = "/v1/users/update/me",
    request_body = UsersUpdateRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "User Me Updated", body = MessageResponseDto),
        (status = 400, description = "Invalid User Me data", body = MessageResponseDto)
    ),
    tag = "Users"
)]
pub async fn put_update_user_me(
	headers: HeaderMap,
	Json(payload): Json<UsersUpdateRequestDto>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![]).await {
		Ok(_) => mutation_update_user_me(headers, Json(payload)).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    put,
    path = "/v1/users/activate/{id}",
    request_body = UsersActiveInactiveRequestDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "User Updated", body = MessageResponseDto),
        (status = 400, description = "Invalid User data", body = MessageResponseDto)
    ),
    tag = "Users"
)]
pub async fn put_activate_user(
	headers: HeaderMap,
	Path(id): Path<String>,
	Json(payload): Json<UsersActiveInactiveRequestDto>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::UpdateUsers]).await
	{
		Ok(_) => mutation_set_active_inactive_user(id, Json(payload)).await,
		Err(response) => response,
	}
}
