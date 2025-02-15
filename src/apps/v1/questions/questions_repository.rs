use axum::{http::StatusCode, response::Response, Json};
use chrono::Utc;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
	prelude::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
	PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::schemas::{TestsActiveModel, TestsColumn, TestsEntity};
use crate::{
	common_response, get_db, success_response, success_response_list,
	MetaRequestDto, MetaResponseDto, ResponseSuccessDto, ResponseSuccessListDto,
};

use super::questions_dto::{
	TestsItemDto, TestsItemListDto, TestsRequestCreateDto, TestsRequestUpdateDto,
};

pub async fn query_get_tests(params: MetaRequestDto) -> Response {
	let db: DatabaseConnection = get_db().await;

	let page = params.page.unwrap_or(1).max(1);
	let per_page = params.per_page.unwrap_or(10).max(1).min(100);
	let search = params.search.unwrap_or_default().to_lowercase();
	let sort_by = params
		.sort_by
		.unwrap_or_else(|| "created_at".to_string())
		.to_lowercase();
	let order = params
		.order
		.unwrap_or_else(|| "desc".to_string())
		.to_lowercase();

	let mut query = TestsEntity::find();

	if !search.is_empty() {
		query = query
			.filter(Expr::col(TestsColumn::Name).ilike(format!("%{}%", search)));
	}

	query = match (sort_by.as_str(), order.as_str()) {
		("name", "asc") => query.order_by_asc(TestsColumn::Name),
		("name", "desc") => query.order_by_desc(TestsColumn::Name),
		("created_at", "asc") => query.order_by_asc(TestsColumn::CreatedAt),
		("created_at", "desc") => query.order_by_desc(TestsColumn::CreatedAt),
		("time_limit", "asc") => query.order_by_asc(TestsColumn::TimeLimit),
		("time_limit", "desc") => query.order_by_desc(TestsColumn::TimeLimit),
		_ => query.order_by_desc(TestsColumn::CreatedAt),
	};

	let paginator = query.paginate(&db, per_page);
	let total_items = match paginator.num_items().await {
		Ok(count) => count,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let tests_data = match paginator.fetch_page(page - 1).await {
		Ok(data) => data,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let data: Vec<TestsItemListDto> = tests_data
		.into_iter()
		.map(|test| TestsItemListDto {
			id: test.id.to_string(),
			name: test.name,
			description: test.description,
			instructions: test.instructions,
			time_limit: test.time_limit,
			created_at: test.created_at.to_string(),
		})
		.collect();

	let response = ResponseSuccessListDto {
		data,
		meta: Some(MetaResponseDto {
			page: Some(page),
			per_page: Some(per_page),
			total: Some(total_items),
		}),
	};

	success_response_list(response)
}

pub async fn query_get_test_by_id(id: String) -> Response {
	let db: DatabaseConnection = get_db().await;

	let test = match TestsEntity::find()
		.filter(TestsColumn::Id.eq(Uuid::parse_str(&id).unwrap_or_default()))
		.one(&db)
		.await
	{
		Ok(Some(test)) => test,
		Ok(None) => return common_response(StatusCode::NOT_FOUND, "Test not found"),
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let test_dto = TestsItemDto {
		id: test.id.to_string(),
		name: test.name,
		description: test.description,
		instructions: test.instructions,
		time_limit: test.time_limit,
		created_at: test.created_at.to_string(),
	};

	let response = ResponseSuccessDto { data: test_dto };
	success_response(response)
}

pub async fn mutation_create_test(payload: Json<TestsRequestCreateDto>) -> Response {
	let db: DatabaseConnection = get_db().await;

	if let Ok(Some(_)) = TestsEntity::find()
		.filter(TestsColumn::Name.eq(payload.name.clone()))
		.one(&db)
		.await
	{
		return common_response(
			StatusCode::CONFLICT,
			"A test with this name already exists",
		);
	}

	let new_test = TestsActiveModel {
		id: Set(Uuid::new_v4()),
		name: Set(payload.name.clone()),
		description: Set(payload.description.clone()),
		instructions: Set(payload.instructions.clone()),
		time_limit: Set(payload.time_limit),
		created_at: Set(Utc::now()),
		..Default::default()
	};

	match new_test.insert(&db).await {
		Ok(_test) => {
			common_response(StatusCode::CREATED, "Test created successfully")
		}
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}

pub async fn mutation_update_test(
	id: String,
	payload: Json<TestsRequestUpdateDto>,
) -> Response {
	let db: DatabaseConnection = get_db().await;

	let test_id = match Uuid::parse_str(&id) {
		Ok(id) => id,
		Err(_) => {
			return common_response(
				StatusCode::BAD_REQUEST,
				"Invalid test ID format",
			)
		}
	};

	let test = match TestsEntity::find()
		.filter(TestsColumn::Id.eq(test_id))
		.one(&db)
		.await
	{
		Ok(Some(test)) => test,
		Ok(None) => return common_response(StatusCode::NOT_FOUND, "Test not found"),
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let mut active_model: TestsActiveModel = test.into();

	if let Some(name) = &payload.name {
		active_model.name = Set(name.clone());
	}
	if let Some(description) = &payload.description {
		active_model.description = Set(Some(description.clone()));
	}
	if let Some(instructions) = &payload.instructions {
		active_model.instructions = Set(Some(instructions.clone()));
	}
	if let Some(time_limit) = payload.time_limit {
		active_model.time_limit = Set(Some(time_limit));
	}

	match active_model.update(&db).await {
		Ok(_updated_test) => {
			common_response(StatusCode::OK, "Test updated successfully")
		}
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}
