use axum::{http::StatusCode, response::Response, Json};
use chrono::Utc;
use futures::future::join_all;
use hyper::HeaderMap;
use sea_orm::{
	prelude::*, sea_query::extension::postgres::PgExpr, ActiveModelTrait,
	ColumnTrait, DatabaseConnection, EntityTrait, JoinType, ModelTrait,
	PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use uuid::Uuid;

use crate::{
	common_response, decode_access_token, get_db,
	schemas::{
		OptionsActiveModel, OptionsColumn, OptionsEntity, QuestionsActiveModel,
		QuestionsColumn, QuestionsEntity, TestsActiveModel, TestsColumn,
		TestsEntity,
	},
	success_response, success_response_list, MetaRequestDto, MetaResponseDto,
	ResponseSuccessDto, ResponseSuccessListDto, SessionsHasTestsColumn,
	SessionsHasTestsEntity, TestAnswersActiveModel, TestAnswersColumn,
	TestAnswersEntity, UsersColumn, UsersEntity,
};

use super::{
	tests_dto::{
		OptionsItemDto, QuestionsItemDto, TestsItemDto, TestsItemListDto,
		TestsRequestCreateDto, TestsRequestUpdateDto,
	},
	TestAnswersItemDto, TestAnswersRequestCreateDto,
};

pub async fn query_get_tests(params: MetaRequestDto) -> Response {
	let db: DatabaseConnection = get_db().await;

	let page = params.page.unwrap_or(1).max(1);
	let per_page = params.per_page.unwrap_or(10).max(1).min(100);
	let search = params.search.unwrap_or_default().to_lowercase();
	let sort_by = params
		.sort_by
		.unwrap_or("created_at".to_string())
		.to_lowercase();
	let order = params.order.unwrap_or("desc".to_string()).to_lowercase();
	let filter = params.filter.unwrap_or_default();
	let filter_by = params.filter_by.unwrap_or_default().to_lowercase();

	let mut query = TestsEntity::find();

	if !search.is_empty() {
		query = query
			.filter(Expr::col(TestsColumn::TestName).ilike(format!("%{}%", search)));
	}

	if filter_by == "session_id" && !filter.is_empty() {
		if let Ok(session_uuid) = Uuid::parse_str(&filter) {
			query = query
				.join(
					JoinType::InnerJoin,
					<TestsEntity as Related<SessionsHasTestsEntity>>::to(),
				)
				.filter(
					Expr::col((
						SessionsHasTestsEntity,
						SessionsHasTestsColumn::SessionId,
					))
					.eq(session_uuid),
				);
		} else {
			return common_response(
				StatusCode::BAD_REQUEST,
				"Invalid session_id format",
			);
		}
	}

	query = match (sort_by.as_str(), order.as_str()) {
		("test_name", "asc") => query.order_by_asc(TestsColumn::TestName),
		("test_name", "desc") => query.order_by_desc(TestsColumn::TestName),
		("created_at", "asc") => query.order_by_asc(TestsColumn::CreatedAt),
		("created_at", "desc") => query.order_by_desc(TestsColumn::CreatedAt),
		("updated_at", "asc") => query.order_by_asc(TestsColumn::UpdatedAt),
		("updated_at", "desc") => query.order_by_desc(TestsColumn::UpdatedAt),
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

	let tests = match paginator.fetch_page(page - 1).await {
		Ok(data) => data,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let data: Vec<TestsItemListDto> = join_all(tests.into_iter().map(|test| {
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

	let questions = QuestionsEntity::find()
		.filter(QuestionsColumn::TestId.eq(test.id))
		.all(&db)
		.await
		.unwrap_or_default();

	let questions_dto: Vec<QuestionsItemDto> =
		join_all(questions.into_iter().map(|q| {
			let db = db.clone();
			async move {
				let options = OptionsEntity::find()
					.filter(OptionsColumn::QuestionId.eq(q.id))
					.all(&db)
					.await
					.unwrap_or_default();
				let options_dto: Vec<OptionsItemDto> = options
					.into_iter()
					.map(|opt| OptionsItemDto {
						id: opt.id.to_string(),
						label: opt.label,
					})
					.collect();

				QuestionsItemDto {
					id: q.id.to_string(),
					question: q.question,
					discussion: q.discussion,
					options: options_dto,
				}
			}
		}))
		.await;

	let test_dto = TestsItemDto {
		id: test.id.to_string(),
		test_name: test.test_name,
		questions: questions_dto,
		created_at: test.created_at.map(|dt| dt.to_string()),
		updated_at: test.updated_at.map(|dt| dt.to_string()),
	};

	let response = ResponseSuccessDto { data: test_dto };
	success_response(response)
}

pub async fn mutation_create_test(payload: Json<TestsRequestCreateDto>) -> Response {
	let db: DatabaseConnection = get_db().await;

	let new_test = TestsActiveModel {
		id: Set(Uuid::new_v4()),
		test_name: Set(payload.test_name.clone()),
		created_at: Set(Some(Utc::now())),
		updated_at: Set(Some(Utc::now())),
		..Default::default()
	};

	let test = match new_test.insert(&db).await {
		Ok(test) => test,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	for question in &payload.questions {
		let new_question = QuestionsActiveModel {
			id: Set(Uuid::new_v4()),
			test_id: Set(test.id),
			question: Set(question.question.clone()),
			discussion: Set(question.discussion.clone()),
			..Default::default()
		};

		let inserted_question = match new_question.insert(&db).await {
			Ok(q) => q,
			Err(err) => {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&err.to_string(),
				)
			}
		};

		for option in &question.options {
			let new_option = OptionsActiveModel {
				id: Set(Uuid::new_v4()),
				question_id: Set(inserted_question.id),
				is_correct: Set(option.is_correct),
				label: Set(option.label.clone()),
				..Default::default()
			};

			if let Err(err) = new_option.insert(&db).await {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&err.to_string(),
				);
			}
		}
	}

	common_response(StatusCode::CREATED, "Test created successfully")
}

pub async fn mutation_update_test(
	id: String,
	payload: Json<TestsRequestUpdateDto>,
) -> Response {
	let db: DatabaseConnection = get_db().await;

	let test_id = match Uuid::parse_str(&id) {
		Ok(uuid) => uuid,
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

	if let Some(test_name) = &payload.test_name {
		active_model.test_name = Set(test_name.clone());
	}
	active_model.updated_at = Set(Some(Utc::now()));

	match active_model.update(&db).await {
		Ok(_test) => common_response(StatusCode::OK, "Test updated successfully"),
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}

pub async fn mutation_delete_test(id: String) -> Response {
	let db: DatabaseConnection = get_db().await;

	let test_id = match Uuid::parse_str(&id) {
		Ok(uuid) => uuid,
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

	match test.delete(&db).await {
		Ok(_) => common_response(StatusCode::OK, "Test deleted successfully"),
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}

pub async fn query_get_test_answer_by_id(id: String) -> Response {
	let db: DatabaseConnection = get_db().await;

	let answer = match TestAnswersEntity::find()
		.filter(TestAnswersColumn::Id.eq(Uuid::parse_str(&id).unwrap_or_default()))
		.one(&db)
		.await
	{
		Ok(Some(answer)) => answer,
		Ok(None) => {
			return common_response(StatusCode::NOT_FOUND, "Test answer not found")
		}
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let dto = TestAnswersItemDto {
		id: answer.id.to_string(),
		user_id: answer.user_id.to_string(),
		test_id: answer.test_id.to_string(),
	};

	let response = ResponseSuccessDto { data: dto };
	success_response(response)
}

pub async fn mutation_create_test_answer(
	headers: HeaderMap,
	payload: Json<TestAnswersRequestCreateDto>,
) -> Response {
	let db: DatabaseConnection = get_db().await;

	let auth_header = match headers.get("Authorization") {
		Some(header) => header.to_str(),
		None => {
			return common_response(StatusCode::FORBIDDEN, "You are not authorized")
		}
	};

	let auth_header = match auth_header {
		Ok(header) => header,
		Err(err) => {
			return common_response(StatusCode::BAD_REQUEST, &err.to_string())
		}
	};

	let mut header_parts = auth_header.split_whitespace();
	let token = match header_parts.nth(1) {
		Some(token) => token,
		None => {
			return common_response(StatusCode::BAD_REQUEST, "Invalid token format")
		}
	};

	let token_data = match decode_access_token(&token) {
		Ok(data) => data,
		Err(err) => {
			return common_response(StatusCode::UNAUTHORIZED, &err.to_string())
		}
	};

	let email = token_data.claims.email;

	let user = match UsersEntity::find()
		.filter(UsersColumn::Email.eq(email))
		.one(&db)
		.await
	{
		Ok(Some(user)) => user,
		Ok(None) => return common_response(StatusCode::NOT_FOUND, "User not found"),
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let new_answer = TestAnswersActiveModel {
		id: Set(Uuid::new_v4()),
		user_id: Set(user.id),
		test_id: Set(Uuid::parse_str(&payload.test_id).unwrap_or_default()),
		..Default::default()
	};

	let answer = match new_answer.insert(&db).await {
		Ok(answer) => answer,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let dto = TestAnswersItemDto {
		id: answer.id.to_string(),
		user_id: answer.user_id.to_string(),
		test_id: answer.test_id.to_string(),
	};

	let response = ResponseSuccessDto { data: dto };
	success_response(response)
}

pub async fn mutation_delete_test_answer(id: String) -> Response {
	let db: DatabaseConnection = get_db().await;
	let answer_id = match Uuid::parse_str(&id) {
		Ok(uuid) => uuid,
		Err(_) => {
			return common_response(
				StatusCode::BAD_REQUEST,
				"Invalid test answer ID",
			)
		}
	};

	let answer = match TestAnswersEntity::find()
		.filter(TestAnswersColumn::Id.eq(answer_id))
		.one(&db)
		.await
	{
		Ok(Some(answer)) => answer,
		Ok(None) => {
			return common_response(StatusCode::NOT_FOUND, "Test answer not found")
		}
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	match answer.delete(&db).await {
		Ok(_) => common_response(StatusCode::OK, "Test answer deleted successfully"),
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}

pub async fn query_test_answer_list(
	session_id: String,
	params: MetaRequestDto,
) -> Response {
	let db: DatabaseConnection = get_db().await;

	let session_uuid = match Uuid::parse_str(&session_id) {
		Ok(uuid) => uuid,
		Err(_) => {
			return common_response(
				StatusCode::BAD_REQUEST,
				"Invalid session_id format",
			)
		}
	};

	let page = params.page.unwrap_or(1).max(1);
	let per_page = params.per_page.unwrap_or(10).max(1).min(100);
	let search = params.search.unwrap_or_default().to_lowercase();
	let sort_by = params.sort_by.unwrap_or("id".to_string()).to_lowercase();
	let order = params.order.unwrap_or("desc".to_string()).to_lowercase();

	let mut query =
		TestAnswersEntity::find().filter(TestAnswersColumn::TestId.eq(session_uuid));

	if !search.is_empty() {
		query = query.filter(
			Expr::cust("CAST(user_id AS TEXT)").ilike(format!("%{}%", search)),
		);
	}

	query = match (sort_by.as_str(), order.as_str()) {
		("id", "asc") => query.order_by_asc(TestAnswersColumn::Id),
		("id", "desc") => query.order_by_desc(TestAnswersColumn::Id),
		("user_id", "asc") => query.order_by_asc(TestAnswersColumn::UserId),
		("user_id", "desc") => query.order_by_desc(TestAnswersColumn::UserId),
		("test_id", "asc") => query.order_by_asc(TestAnswersColumn::TestId),
		("test_id", "desc") => query.order_by_desc(TestAnswersColumn::TestId),
		_ => query.order_by_desc(TestAnswersColumn::Id),
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

	let answers = match paginator.fetch_page(page - 1).await {
		Ok(data) => data,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let answers_dto: Vec<_> = answers
		.into_iter()
		.map(|answer| TestAnswersItemDto {
			id: answer.id.to_string(),
			user_id: answer.user_id.to_string(),
			test_id: answer.test_id.to_string(),
		})
		.collect();

	let response = ResponseSuccessListDto {
		data: answers_dto,
		meta: Some(MetaResponseDto {
			page: Some(page),
			per_page: Some(per_page),
			total: Some(total_items),
		}),
	};

	success_response_list(response)
}
