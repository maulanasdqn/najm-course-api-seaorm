use super::roles_dto::{RolesDetailResponseDto, RolesItemDto, RolesListResponseDto};
use crate::{
    apps::v1::permissions::permissions_dto::PermissionsItemDto,
    get_version,
    libs::database::{
        get_db,
        schemas::{
            app_permissions_schema::Entity as Permission,
            app_roles_permissions_schema::{
                Column as RolePermissionColumn, Entity as RolePermission,
            },
            app_roles_schema::{Column as RoleColumn, Entity as Role, Model as RoleModel},
        },
    },
    utils::{
        dto::{MetaRequestDto, MetaResponseDto},
        error::AppError,
    },
};
use axum::Json;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
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
