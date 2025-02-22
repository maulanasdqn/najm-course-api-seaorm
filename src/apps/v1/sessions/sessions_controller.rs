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
	mutation_delete_session, mutation_update_session,
	sessions_dto::{SessionsRequestCreateDto, SessionsRequestUpdateDto},
	sessions_repository::{
		mutation_create_session, query_get_session_by_id, query_get_sessions,
	},
	SessionsItemDto, SessionsItemListDto,
};

#[utoipa::path(
    get,
    path = "/v1/sessions",
    params(MetaRequestDto),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Sessions", body = ResponseSuccessListDto<SessionsItemListDto>),
        (status = 400, description = "Invalid Sessions data", body = MessageResponseDto)
    ),
    tag = "Sessions"
)]
pub async fn get_sessions(
	headers: HeaderMap,
	Query(params): Query<MetaRequestDto>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::ReadListSessions])
		.await
	{
		Ok(_) => query_get_sessions(params).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    get,
    path = "/v1/sessions/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Detail Session", body = ResponseSuccessDto<SessionsItemDto>),
        (status = 400, description = "Invalid Session data", body = MessageResponseDto)
    ),
    tag = "Sessions"
)]
pub async fn get_detail_session(
	headers: HeaderMap,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::ReadDetailSessions])
		.await
	{
		Ok(_) => query_get_session_by_id(id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    post,
    path = "/v1/sessions/create",
    request_body = SessionsRequestCreateDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Session Created", body = MessageResponseDto),
        (status = 400, description = "Invalid Session data", body = MessageResponseDto)
    ),
    tag = "Sessions"
)]
pub async fn post_create_session(
	headers: HeaderMap,
	Json(payload): Json<SessionsRequestCreateDto>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::CreateSessions])
		.await
	{
		Ok(_) => mutation_create_session(Json(payload)).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    put,
    path = "/v1/sessions/update/{id}",
    request_body = SessionsRequestUpdateDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Session Updated", body = MessageResponseDto),
        (status = 400, description = "Invalid Session data", body = MessageResponseDto)
    ),
    tag = "Sessions"
)]
pub async fn put_update_session(
	headers: HeaderMap,
	Path(id): Path<String>,
	Json(payload): Json<SessionsRequestUpdateDto>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::UpdateSessions])
		.await
	{
		Ok(_) => mutation_update_session(id, Json(payload)).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    delete,
    path = "/v1/sessions/delete/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Session Deleted", body = MessageResponseDto),
        (status = 400, description = "Invalid Session data", body = MessageResponseDto)
    ),
    tag = "Sessions"
)]
pub async fn delete_session(
	headers: HeaderMap,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::DeleteSessions])
		.await
	{
		Ok(_) => mutation_delete_session(id).await,
		Err(response) => response,
	}
}
