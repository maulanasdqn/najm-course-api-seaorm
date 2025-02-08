use axum::{response::Response, Json};
use hyper::StatusCode;
use sea_orm::{
	prelude::Expr, sea_query::extension::postgres::PgExpr, ActiveModelTrait,
	ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait,
	QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
	common_response, get_db,
	schemas::{PermissionsActiveModel, PermissionsColumn, PermissionsEntity},
	success_response, success_response_list, MetaRequestDto, MetaResponseDto,
	ResponseSuccessDto, ResponseSuccessListDto,
};

use super::{PermissionsItemDto, PermissionsRequestDto};

pub async fn mutation_create_permission(
	payload: Json<PermissionsRequestDto>,
) -> Response {
	let db: DatabaseConnection = get_db().await;

	if let Ok(Some(_)) = PermissionsEntity::find()
		.filter(PermissionsColumn::Name.eq(payload.name.clone()))
		.one(&db)
		.await
	{
		return common_response(
			StatusCode::CONFLICT,
			"A permission with this name already exists",
		);
	}

	let new_permission = PermissionsActiveModel {
		id: Set(Uuid::new_v4()),
		name: Set(payload.name.clone()),
		created_at: Set(Some(chrono::Utc::now())),
		updated_at: Set(Some(chrono::Utc::now())),
	};

	match new_permission.insert(&db).await {
		Ok(_) => {
			common_response(StatusCode::CREATED, "Permission created successfully")
		}
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}

pub async fn query_get_permissions(params: MetaRequestDto) -> Response {
	let db: DatabaseConnection = get_db().await;

	let page = params.page.unwrap_or(1).max(1);
	let per_page = params.per_page.unwrap_or(10).max(1).min(100);
	let search = params.search.unwrap_or_default().to_lowercase();
	let sort_by = params
		.sort_by
		.unwrap_or("created_at".to_string())
		.to_lowercase();
	let order = params.order.unwrap_or("desc".to_string()).to_lowercase();

	let mut query = PermissionsEntity::find();

	if !search.is_empty() {
		query = query.filter(
			Expr::col(PermissionsColumn::Name).ilike(format!("%{}%", search)),
		);
	}

	query = match (sort_by.as_str(), order.as_str()) {
		("name", "asc") => query.order_by_asc(PermissionsColumn::Name),
		("name", "desc") => query.order_by_desc(PermissionsColumn::Name),
		("created_at", "asc") => query.order_by_asc(PermissionsColumn::CreatedAt),
		("created_at", "desc") => query.order_by_desc(PermissionsColumn::CreatedAt),
		("updated_at", "asc") => query.order_by_asc(PermissionsColumn::UpdatedAt),
		("updated_at", "desc") => query.order_by_desc(PermissionsColumn::UpdatedAt),
		_ => query.order_by_desc(PermissionsColumn::CreatedAt),
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

	let permissions = match paginator.fetch_page(page - 1).await {
		Ok(data) => data,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let data: Vec<PermissionsItemDto> = permissions
		.into_iter()
		.map(|permission| PermissionsItemDto {
			id: permission.id.to_string(),
			name: permission.name,
			created_at: permission.created_at.map(|dt| dt.to_string()),
			updated_at: permission.updated_at.map(|dt| dt.to_string()),
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

pub async fn query_get_permission_by_id(id: String) -> Response {
	let db: DatabaseConnection = get_db().await;

	let permission = match PermissionsEntity::find()
		.filter(PermissionsColumn::Id.eq(Uuid::parse_str(&id).unwrap_or_default()))
		.one(&db)
		.await
	{
		Ok(Some(permission)) => permission,
		Ok(None) => {
			return common_response(StatusCode::NOT_FOUND, "Permission not found")
		}
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let permission_dto = PermissionsItemDto {
		id: permission.id.to_string(),
		name: permission.name,
		created_at: permission.created_at.map(|dt| dt.to_string()),
		updated_at: permission.updated_at.map(|dt| dt.to_string()),
	};

	let response = ResponseSuccessDto {
		data: permission_dto,
	};

	success_response(response)
}

pub async fn mutation_update_permission(
	id: String,
	payload: Json<PermissionsRequestDto>,
) -> Response {
	let db: DatabaseConnection = get_db().await;

	let permission_id = match Uuid::parse_str(&id) {
		Ok(id) => id,
		Err(_) => {
			return common_response(
				StatusCode::BAD_REQUEST,
				"Invalid permission ID format",
			)
		}
	};

	let permission = match PermissionsEntity::find()
		.filter(PermissionsColumn::Id.eq(permission_id))
		.one(&db)
		.await
	{
		Ok(Some(permission)) => permission,
		Ok(None) => {
			return common_response(StatusCode::NOT_FOUND, "Permission not found")
		}
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let mut active_model: PermissionsActiveModel = permission.into();

	if !payload.name.is_empty() {
		active_model.name = Set(payload.name.clone());
	}
	active_model.updated_at = Set(Some(chrono::Utc::now()));

	match active_model.update(&db).await {
		Ok(_) => common_response(StatusCode::OK, "Permission updated successfully"),
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}

pub async fn mutation_delete_permission(id: String) -> Response {
	let db: DatabaseConnection = get_db().await;

	let permission_id = match Uuid::parse_str(&id) {
		Ok(id) => id,
		Err(_) => {
			return common_response(
				StatusCode::BAD_REQUEST,
				"Invalid permission ID format",
			)
		}
	};

	let permission = match PermissionsEntity::find()
		.filter(PermissionsColumn::Id.eq(permission_id))
		.one(&db)
		.await
	{
		Ok(Some(permission)) => permission,
		Ok(None) => {
			return common_response(StatusCode::NOT_FOUND, "Permission not found")
		}
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	match permission.delete(&db).await {
		Ok(_) => common_response(StatusCode::OK, "Permission deleted successfully"),
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}
