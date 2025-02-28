use axum::{http::StatusCode, response::Response, Json};
use chrono::Utc;
use futures::future::join_all;
use sea_orm::{
	prelude::*, sea_query::extension::postgres::PgExpr, ActiveModelTrait,
	ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait,
	QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
	common_response, get_db,
	schemas::{
		TestSessionsActiveModel, TestSessionsColumn, TestSessionsEntity, TestsEntity,
	},
	success_response, success_response_list, MetaRequestDto, MetaResponseDto,
	QuestionsColumn, QuestionsEntity, ResponseSuccessDto, ResponseSuccessListDto,
	TestsItemListDto,
};

use super::sessions_dto::{
	SessionsItemDto, SessionsItemListDto, SessionsRequestCreateDto,
	SessionsRequestUpdateDto,
};

pub async fn query_get_sessions(params: MetaRequestDto) -> Response {
	let db: DatabaseConnection = get_db().await;

	let page = params.page.unwrap_or(1).max(1);
	let per_page = params.per_page.unwrap_or(10).max(1).min(100);
	let search = params.search.unwrap_or_default().to_lowercase();
	let sort_by = params
		.sort_by
		.unwrap_or("created_at".to_string())
		.to_lowercase();
	let order = params.order.unwrap_or("desc".to_string()).to_lowercase();

	let mut query = TestSessionsEntity::find();

	if !search.is_empty() {
		query = query.filter(
			Expr::col(TestSessionsColumn::SessionName)
				.ilike(format!("%{}%", search)),
		);
	}

	query = match (sort_by.as_str(), order.as_str()) {
		("session_name", "asc") => {
			query.order_by_asc(TestSessionsColumn::SessionName)
		}
		("session_name", "desc") => {
			query.order_by_desc(TestSessionsColumn::SessionName)
		}
		("created_at", "asc") => query.order_by_asc(TestSessionsColumn::CreatedAt),
		("created_at", "desc") => query.order_by_desc(TestSessionsColumn::CreatedAt),
		("updated_at", "asc") => query.order_by_asc(TestSessionsColumn::UpdatedAt),
		("updated_at", "desc") => query.order_by_desc(TestSessionsColumn::UpdatedAt),
		_ => query.order_by_desc(TestSessionsColumn::CreatedAt),
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

	let sessions = match paginator.fetch_page(page - 1).await {
		Ok(data) => data,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let data: Vec<SessionsItemListDto> =
		futures::future::join_all(sessions.into_iter().map(|session| {
			let db = db.clone();
			async move {
				let test_count = TestsEntity::find()
					.filter(crate::schemas::TestsColumn::SessionId.eq(session.id))
					.count(&db)
					.await
					.unwrap_or(0);

				SessionsItemListDto {
					id: session.id.to_string(),
					session_name: session.session_name,
					student_type: session.student_type,
					description: session.description,
					is_active: session.is_active,
					test_count,
					created_at: session.created_at.map(|dt| dt.to_string()),
					updated_at: session.updated_at.map(|dt| dt.to_string()),
				}
			}
		}))
		.await;

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

pub async fn query_get_session_by_id(id: String) -> Response {
	let db: DatabaseConnection = get_db().await;

	let session = match TestSessionsEntity::find()
		.filter(TestSessionsColumn::Id.eq(Uuid::parse_str(&id).unwrap_or_default()))
		.one(&db)
		.await
	{
		Ok(Some(session)) => session,
		Ok(None) => {
			return common_response(StatusCode::NOT_FOUND, "Session not found")
		}
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let tests_entities = match TestsEntity::find()
		.filter(crate::schemas::TestsColumn::SessionId.eq(session.id))
		.all(&db)
		.await
	{
		Ok(tests) => tests,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let tests_dto_futures = tests_entities.into_iter().map(|test| {
		let db = db.clone();
		async move {
			let question_count = QuestionsEntity::find()
				.filter(QuestionsColumn::TestId.eq(test.id))
				.count(&db)
				.await
				.unwrap_or(0);
			TestsItemListDto {
				id: test.id.to_string(),
				test_name: test.test_name,
				question_count,
				created_at: test.created_at.map(|dt| dt.to_string()),
				updated_at: test.updated_at.map(|dt| dt.to_string()),
			}
		}
	});
	let tests_dto: Vec<TestsItemListDto> = join_all(tests_dto_futures).await;

	let session_dto = SessionsItemDto {
		id: session.id.to_string(),
		session_name: session.session_name,
		student_type: session.student_type,
		description: session.description,
		is_active: session.is_active,
		tests: tests_dto,
		created_at: session.created_at.map(|dt| dt.to_string()),
		updated_at: session.updated_at.map(|dt| dt.to_string()),
	};

	let response = ResponseSuccessDto { data: session_dto };
	success_response(response)
}

pub async fn mutation_create_session(
	payload: Json<SessionsRequestCreateDto>,
) -> Response {
	let db: DatabaseConnection = get_db().await;

	let new_session = TestSessionsActiveModel {
		id: Set(Uuid::new_v4()),
		session_name: Set(payload.session_name.clone()),
		description: Set(payload.description.clone()),
		is_active: Set(payload.is_active),
		created_at: Set(Some(Utc::now())),
		updated_at: Set(Some(Utc::now())),
		..Default::default()
	};

	match new_session.insert(&db).await {
		Ok(_session) => {
			common_response(StatusCode::CREATED, "Session created successfully")
		}
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}

pub async fn mutation_update_session(
	id: String,
	payload: Json<SessionsRequestUpdateDto>,
) -> Response {
	let db: DatabaseConnection = get_db().await;

	let session_id = match Uuid::parse_str(&id) {
		Ok(id) => id,
		Err(_) => {
			return common_response(
				StatusCode::BAD_REQUEST,
				"Invalid session ID format",
			)
		}
	};

	let session = match TestSessionsEntity::find()
		.filter(TestSessionsColumn::Id.eq(session_id))
		.one(&db)
		.await
	{
		Ok(Some(session)) => session,
		Ok(None) => {
			return common_response(StatusCode::NOT_FOUND, "Session not found")
		}
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let mut active_model: TestSessionsActiveModel = session.into();

	if payload.session_name != "" {
		active_model.session_name = Set(payload.session_name.clone());
	}

	if payload.student_type != "" {
		active_model.student_type = Set(Some(payload.student_type.clone()));
	}

	if payload.description != "" {
		active_model.description = Set(payload.description.clone());
	}

	active_model.is_active = Set(payload.is_active);

	active_model.updated_at = Set(Some(Utc::now()));

	match active_model.update(&db).await {
		Ok(_session) => {
			common_response(StatusCode::OK, "Session updated successfully")
		}
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}

pub async fn mutation_delete_session(id: String) -> Response {
	let db: DatabaseConnection = get_db().await;

	let session_id = match Uuid::parse_str(&id) {
		Ok(id) => id,
		Err(_) => {
			return common_response(
				StatusCode::BAD_REQUEST,
				"Invalid session ID format",
			)
		}
	};

	let session = match TestSessionsEntity::find()
		.filter(TestSessionsColumn::Id.eq(session_id))
		.one(&db)
		.await
	{
		Ok(Some(session)) => session,
		Ok(None) => {
			return common_response(StatusCode::NOT_FOUND, "Session not found")
		}
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	match session.delete(&db).await {
		Ok(_) => common_response(StatusCode::OK, "Session deleted successfully"),
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}
