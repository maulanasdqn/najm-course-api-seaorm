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
	mutation_create_test_answer, mutation_delete_test_answer,
	query_get_test_answer_by_id,
	tests_dto::{TestsRequestCreateDto, TestsRequestUpdateDto},
	tests_repository::{
		mutation_create_test, mutation_delete_test, mutation_update_test,
		query_get_test_by_id, query_get_tests,
	},
	TestAnswersItemDto, TestAnswersRequestCreateDto, TestsItemDto, TestsItemListDto,
};

#[utoipa::path(
    get,
    path = "/v1/tests",
    params(MetaRequestDto),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 200, description = "List Tests", body = ResponseSuccessListDto<TestsItemListDto>),
        (status = 400, description = "Invalid Tests data", body = MessageResponseDto)
    ),
    tag = "Tests"
)]
pub async fn get_tests(
	headers: HeaderMap,
	Query(params): Query<MetaRequestDto>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::ReadListTests])
		.await
	{
		Ok(_) => query_get_tests(params).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    get,
    path = "/v1/tests/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 200, description = "Detail Test", body = ResponseSuccessDto<TestsItemDto>),
        (status = 400, description = "Invalid Test data", body = MessageResponseDto)
    ),
    tag = "Tests"
)]
pub async fn get_detail_test(
	headers: HeaderMap,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::ReadDetailTests])
		.await
	{
		Ok(_) => query_get_test_by_id(id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    post,
    path = "/v1/tests/create",
    request_body = TestsRequestCreateDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Test Created", body = MessageResponseDto),
        (status = 400, description = "Invalid Test data", body = MessageResponseDto)
    ),
    tag = "Tests"
)]
pub async fn post_create_test(
	headers: HeaderMap,
	Json(payload): Json<TestsRequestCreateDto>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::CreateTests]).await
	{
		Ok(_) => mutation_create_test(Json(payload)).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    put,
    path = "/v1/tests/update/{id}",
    request_body = TestsRequestUpdateDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 200, description = "Test Updated", body = MessageResponseDto),
        (status = 400, description = "Invalid Test data", body = MessageResponseDto)
    ),
    tag = "Tests"
)]
pub async fn put_update_test(
	headers: HeaderMap,
	Path(id): Path<String>,
	Json(payload): Json<TestsRequestUpdateDto>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::UpdateTests]).await
	{
		Ok(_) => mutation_update_test(id, Json(payload)).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    delete,
    path = "/v1/tests/delete/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 200, description = "Test Deleted", body = MessageResponseDto),
        (status = 400, description = "Invalid Test data", body = MessageResponseDto)
    ),
    tag = "Tests"
)]
pub async fn delete_test(
	headers: HeaderMap,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::DeleteTests]).await
	{
		Ok(_) => mutation_delete_test(id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    get,
    path = "/v1/tests/answer/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 200, description = "Detail Test Answer", body = ResponseSuccessDto<TestAnswersItemDto>),
        (status = 400, description = "Invalid Test Answer data", body = MessageResponseDto)
    ),
    tag = "Tests"
)]
pub async fn get_test_answer(
	headers: HeaderMap,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::ReadDetailTests])
		.await
	{
		Ok(_) => query_get_test_answer_by_id(id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    post,
    path = "/v1/tests/answer/create",
    request_body = TestAnswersRequestCreateDto,
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "Test Answer Created", body = ResponseSuccessDto<TestAnswersItemDto>),
        (status = 400, description = "Invalid Test Answer data", body = MessageResponseDto)
    ),
    tag = "Tests"
)]
pub async fn post_create_test_answer(
	headers: HeaderMap,
	Json(payload): Json<TestAnswersRequestCreateDto>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::CreateTests]).await
	{
		Ok(_) => mutation_create_test_answer(headers, Json(payload)).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    delete,
    path = "/v1/tests/answer/delete/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 200, description = "Test Answer Deleted", body = MessageResponseDto),
        (status = 400, description = "Invalid Test Answer data", body = MessageResponseDto)
    ),
    tag = "Tests"
)]
pub async fn delete_test_answer(
	headers: HeaderMap,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_middleware(&headers, vec![PermissionsEnum::DeleteTests]).await
	{
		Ok(_) => mutation_delete_test_answer(id).await,
		Err(response) => response,
	}
}
