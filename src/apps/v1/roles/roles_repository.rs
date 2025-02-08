use axum::{http::StatusCode, response::Response, Json};
use sea_orm::{
	prelude::Expr, sea_query::extension::postgres::PgExpr, ActiveModelTrait,
	ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait,
	QueryFilter, QueryOrder, QuerySelect, Set,
};
use uuid::Uuid;

use crate::{
	common_response, get_db,
	permissions::PermissionsItemDto,
	schemas::{
		PermissionsEntity, RolesActiveModel, RolesColumn, RolesEntity,
		RolesPermissionsActiveModel, RolesPermissionsColumn, RolesPermissionsEntity,
	},
	success_response, success_response_list, MetaRequestDto, MetaResponseDto,
	ResponseSuccessDto, ResponseSuccessListDto,
};

use super::{
	roles_dto::{RolesItemDto, RolesRequestCreateDto},
	RolesItemListDto, RolesRequestUpdateDto,
};

pub async fn query_get_roles(params: MetaRequestDto) -> Response {
	let db: DatabaseConnection = get_db().await;

	let page = params.page.unwrap_or(1).max(1);
	let per_page = params.per_page.unwrap_or(10).max(1).min(100);
	let search = params.search.unwrap_or_default().to_lowercase();
	let sort_by = params
		.sort_by
		.unwrap_or("created_at".to_string())
		.to_lowercase();
	let order = params.order.unwrap_or("desc".to_string()).to_lowercase();

	let mut query = RolesEntity::find();

	if !search.is_empty() {
		query = query
			.filter(Expr::col(RolesColumn::Name).ilike(format!("%{}%", search)));
	}

	query = match (sort_by.as_str(), order.as_str()) {
		("name", "asc") => query.order_by_asc(RolesColumn::Name),
		("name", "desc") => query.order_by_desc(RolesColumn::Name),
		("created_at", "asc") => query.order_by_asc(RolesColumn::CreatedAt),
		("created_at", "desc") => query.order_by_desc(RolesColumn::CreatedAt),
		("updated_at", "asc") => query.order_by_asc(RolesColumn::UpdatedAt),
		("updated_at", "desc") => query.order_by_desc(RolesColumn::UpdatedAt),
		_ => query.order_by_desc(RolesColumn::CreatedAt),
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

	let roles = match paginator.fetch_page(page - 1).await {
		Ok(data) => data,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let data: Vec<RolesItemListDto> = roles
		.into_iter()
		.map(|role| RolesItemListDto {
			id: role.id.to_string(),
			name: role.name,
			created_at: role.created_at.map(|dt| dt.to_string()),
			updated_at: role.updated_at.map(|dt| dt.to_string()),
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

pub async fn query_get_role_by_id(id: String) -> Response {
	let db: DatabaseConnection = get_db().await;

	let role = match RolesEntity::find()
		.filter(RolesColumn::Id.eq(Uuid::parse_str(&id).unwrap_or_default()))
		.one(&db)
		.await
	{
		Ok(Some(role)) => role,
		Ok(None) => return common_response(StatusCode::NOT_FOUND, "Role not found"),
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let permissions = match RolesPermissionsEntity::find()
		.filter(RolesPermissionsColumn::RoleId.eq(role.id))
		.find_also_related(PermissionsEntity)
		.all(&db)
		.await
	{
		Ok(data) => data,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let permissions_dto = permissions
		.into_iter()
		.filter_map(|(_, permission)| permission)
		.map(|permission| PermissionsItemDto {
			id: permission.id.to_string(),
			name: permission.name,
			created_at: permission.created_at.map(|dt| dt.to_string()),
			updated_at: permission.updated_at.map(|dt| dt.to_string()),
		})
		.collect::<Vec<PermissionsItemDto>>();

	let role_dto = RolesItemDto {
		id: role.id.to_string(),
		name: role.name,
		created_at: role.created_at.map(|dt| dt.to_string()),
		updated_at: role.updated_at.map(|dt| dt.to_string()),
		permissions: permissions_dto,
	};

	let response = ResponseSuccessDto { data: role_dto };
	success_response(response)
}

pub async fn mutation_create_role(payload: Json<RolesRequestCreateDto>) -> Response {
	let db: DatabaseConnection = get_db().await;

	if let Ok(Some(_)) = RolesEntity::find()
		.filter(RolesColumn::Name.eq(payload.name.clone()))
		.one(&db)
		.await
	{
		return common_response(
			StatusCode::CONFLICT,
			"A role with this name already exists",
		);
	}

	let new_role = RolesActiveModel {
		id: Set(Uuid::new_v4()),
		name: Set(payload.name.clone()),
		created_at: Set(Some(chrono::Utc::now())),
		updated_at: Set(Some(chrono::Utc::now())),
	};

	let role = match new_role.insert(&db).await {
		Ok(role) => role,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	if let Some(permission_ids) = &payload.permissions {
		for permission_id in permission_ids {
			let role_permission = RolesPermissionsActiveModel {
				id: Set(Uuid::new_v4()),
				role_id: Set(role.id),
				permission_id: Set(Uuid::parse_str(permission_id)
					.unwrap_or_else(|_| Uuid::new_v4())),
			};

			if let Err(err) = role_permission.insert(&db).await {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&err.to_string(),
				);
			}
		}
	}

	common_response(StatusCode::CREATED, "Role created successfully")
}

pub async fn mutation_update_role(
	id: String,
	payload: Json<RolesRequestUpdateDto>,
) -> Response {
	let db: DatabaseConnection = get_db().await;

	let role_id = match Uuid::parse_str(&id) {
		Ok(id) => id,
		Err(_) => {
			return common_response(
				StatusCode::BAD_REQUEST,
				"Invalid role ID format",
			)
		}
	};

	let role = match RolesEntity::find()
		.filter(RolesColumn::Id.eq(role_id))
		.one(&db)
		.await
	{
		Ok(Some(role)) => role,
		Ok(None) => return common_response(StatusCode::NOT_FOUND, "Role not found"),
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let mut active_model: RolesActiveModel = role.into();

	if let Some(name) = &payload.name {
		active_model.name = Set(name.clone());
	}
	active_model.updated_at = Set(Some(chrono::Utc::now()));

	let updated_role = match active_model.update(&db).await {
		Ok(role) => role,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	if let Some(permission_ids) = &payload.permissions {
		let existing_permissions: Vec<Uuid> = match RolesPermissionsEntity::find()
			.filter(RolesPermissionsColumn::RoleId.eq(updated_role.id))
			.select_only()
			.column(RolesPermissionsColumn::PermissionId)
			.into_tuple::<Uuid>()
			.all(&db)
			.await
		{
			Ok(permissions) => permissions,
			Err(err) => {
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&format!("Failed to fetch existing permissions: {}", err),
				);
			}
		};

		let new_permission_ids: Vec<Uuid> = permission_ids
			.iter()
			.filter_map(|p| Uuid::parse_str(p).ok())
			.collect();

		let permissions_to_remove: Vec<Uuid> = existing_permissions
			.iter()
			.filter(|perm| !new_permission_ids.contains(perm))
			.cloned()
			.collect();

		if !permissions_to_remove.is_empty() {
			if let Err(err) = RolesPermissionsEntity::delete_many()
				.filter(
					RolesPermissionsColumn::RoleId.eq(updated_role.id).and(
						RolesPermissionsColumn::PermissionId
							.is_in(permissions_to_remove),
					),
				)
				.exec(&db)
				.await
			{
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&format!("Failed to remove old permissions: {}", err),
				);
			}
		}

		for permission_id in &new_permission_ids {
			if !existing_permissions.contains(permission_id) {
				let role_permission = RolesPermissionsActiveModel {
					id: Set(Uuid::new_v4()),
					role_id: Set(updated_role.id),
					permission_id: Set(*permission_id),
				};

				if let Err(err) = role_permission.insert(&db).await {
					return common_response(
						StatusCode::INTERNAL_SERVER_ERROR,
						&err.to_string(),
					);
				}
			}
		}
	}

	common_response(StatusCode::OK, "Role updated successfully")
}

pub async fn mutation_delete_role(id: String) -> Response {
	let db: DatabaseConnection = get_db().await;

	let role_id = match Uuid::parse_str(&id) {
		Ok(id) => id,
		Err(_) => {
			return common_response(
				StatusCode::BAD_REQUEST,
				"Invalid role ID format",
			)
		}
	};

	let role = match RolesEntity::find()
		.filter(RolesColumn::Id.eq(role_id))
		.one(&db)
		.await
	{
		Ok(Some(role)) => role,
		Ok(None) => return common_response(StatusCode::NOT_FOUND, "Role not found"),
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	match role.delete(&db).await {
		Ok(_) => {
			if let Err(err) = RolesPermissionsEntity::delete_many()
				.filter(RolesPermissionsColumn::RoleId.eq(role_id))
				.exec(&db)
				.await
			{
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&format!("Failed to delete associated permissions: {}", err),
				);
			}
			common_response(StatusCode::OK, "Role deleted successfully")
		}
		Err(err) => {
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}
