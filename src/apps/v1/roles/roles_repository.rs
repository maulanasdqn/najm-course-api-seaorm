use super::roles_dto::{
    RolesDetailResponseDto, RolesItemDto, RolesListResponseDto, RolesRequestDto,
};
use crate::{
    apps::v1::permissions::permissions_dto::PermissionsItemDto,
    get_version,
    libs::database::{
        get_db,
        schemas::{
            app_permissions_schema::Entity as Permission,
            app_roles_permissions_schema::{
                ActiveModel as RolePermissionActiveModel, Column as RolePermissionColumn,
                Entity as RolePermission,
            },
            app_roles_schema::{
                ActiveModel as RoleActiveModel, Column as RoleColumn, Entity as Role,
                Model as RoleModel,
            },
        },
    },
    utils::{
        dto::{MetaRequestDto, MetaResponseDto},
        error::AppError,
    },
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use serde_json::json;
use uuid::Uuid;

pub async fn query_get_roles(params: MetaRequestDto) -> Json<RolesListResponseDto> {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(10).max(1).min(100);

    let paginator = Role::find().paginate(&db, per_page);

    let total_items = paginator.num_items().await.unwrap_or(0);

    let roles: Vec<RoleModel> = paginator.fetch_page(page - 1).await.unwrap_or_default();

    let data: Vec<RolesItemDto> = roles
        .into_iter()
        .map(|role| RolesItemDto {
            id: role.id.to_string(),
            name: role.name,
            created_at: role.created_at.map(|dt| dt.to_string()),
            updated_at: role.updated_at.map(|dt| dt.to_string()),
            permissions: vec![],
        })
        .collect();

    Json(RolesListResponseDto {
        data,
        meta: MetaResponseDto {
            page: Some(page),
            per_page: Some(per_page),
            total: Some(total_items),
        },
        version,
    })
}

pub async fn query_get_role_by_id(id: Uuid) -> Result<Json<RolesDetailResponseDto>, AppError> {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    let role = Role::find()
        .filter(RoleColumn::Id.eq(id))
        .one(&db)
        .await?
        .ok_or(AppError::NotFound)?;

    let permissions = RolePermission::find()
        .filter(RolePermissionColumn::RoleId.eq(id))
        .find_also_related(Permission)
        .all(&db)
        .await?
        .into_iter()
        .filter_map(|(_, permission)| permission)
        .map(|permission| PermissionsItemDto {
            id: permission.id.to_string(),
            name: permission.name,
            created_at: permission.created_at.map(|dt| dt.to_string()),
            updated_at: permission.updated_at.map(|dt| dt.to_string()),
        })
        .collect::<Vec<PermissionsItemDto>>();

    let role_detail = RolesItemDto {
        id: role.id.to_string(),
        name: role.name,
        created_at: role.created_at.map(|dt| dt.to_string()),
        updated_at: role.updated_at.map(|dt| dt.to_string()),
        permissions,
    };

    Ok(Json(RolesDetailResponseDto {
        data: role_detail,
        version,
    }))
}

pub async fn mutation_create_role(payload: Json<RolesRequestDto>) -> impl IntoResponse {
    let db: DatabaseConnection = get_db().await;

    let version = match get_version() {
        Ok(ver) => ver,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to fetch application version" })),
            )
                .into_response();
        }
    };

    if let Ok(Some(_)) = Role::find()
        .filter(RoleColumn::Name.eq(payload.name.clone()))
        .one(&db)
        .await
    {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "message": "A role with this name already exists",
                "version": version,
            })),
        )
            .into_response();
    }

    let new_role = RoleActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(payload.name.clone()),
        created_at: Set(Some(chrono::Utc::now())),
        updated_at: Set(Some(chrono::Utc::now())),
    };

    let role = match new_role.insert(&db).await {
        Ok(role) => role,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "Failed to create role",
                    "version": version,
                    "error": err.to_string(),
                })),
            )
                .into_response();
        }
    };

    if let Some(permission_ids) = &payload.permissions {
        for permission_id in permission_ids {
            let role_permission = RolePermissionActiveModel {
                id: Set(Uuid::new_v4()),
                role_id: Set(role.id),
                permission_id: Set(
                    Uuid::parse_str(permission_id).unwrap_or_else(|_| Uuid::new_v4())
                ),
            };

            if let Err(err) = role_permission.insert(&db).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "Failed to associate permissions with the role",
                        "version": version,
                        "error": err.to_string(),
                    })),
                )
                    .into_response();
            }
        }
    }

    (
        StatusCode::CREATED,
        Json(json!({
            "message": "Role created successfully",
            "version": version,
        })),
    )
        .into_response()
}
