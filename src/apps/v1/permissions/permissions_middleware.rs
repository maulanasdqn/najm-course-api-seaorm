use axum::response::Response;
use hyper::{HeaderMap, StatusCode};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    common_response, decode_access_token, get_db,
    schemas::{
        PermissionsEntity, RolesEntity, RolesPermissionsColumn, RolesPermissionsEntity,
        UsersColumn, UsersEntity,
    },
};

use super::PermissionsEnum;

pub async fn permissions_middleware(
    headers: HeaderMap,
    required_permissions: Vec<PermissionsEnum>,
) -> Result<(), Response> {
    let db = get_db().await;

    let auth_header = match headers.get("Authorization") {
        Some(header) => header.to_str(),
        None => {
            return Err(common_response(
                StatusCode::FORBIDDEN,
                "You are not authorized",
            ));
        }
    };

    let auth_header = match auth_header {
        Ok(header) => header,
        Err(_) => {
            return Err(common_response(
                StatusCode::BAD_REQUEST,
                "Invalid header format",
            ));
        }
    };

    let mut header_parts = auth_header.split_whitespace();

    let token = match header_parts.nth(1) {
        Some(token) => token,
        None => {
            return Err(common_response(
                StatusCode::BAD_REQUEST,
                "Invalid token format",
            ));
        }
    };

    let token_data = match decode_access_token(token.to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(common_response(
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token",
            ));
        }
    };

    let email = token_data.claims.email;

    let user = match UsersEntity::find()
        .filter(UsersColumn::Email.eq(email))
        .find_also_related(RolesEntity)
        .one(&db)
        .await
    {
        Ok(Some((user, Some(role)))) => (user, role),
        Ok(Some((_, None))) => {
            return Err(common_response(
                StatusCode::FORBIDDEN,
                "User does not have an assigned role",
            ));
        }
        Ok(None) => {
            return Err(common_response(StatusCode::NOT_FOUND, "User not found"));
        }
        Err(err) => {
            return Err(common_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &err.to_string(),
            ));
        }
    };

    let (_, role) = user;

    let permissions = RolesPermissionsEntity::find()
        .filter(RolesPermissionsColumn::RoleId.eq(role.id))
        .find_also_related(PermissionsEntity)
        .all(&db)
        .await
        .unwrap_or_default();

    let role_permissions: Vec<String> = permissions
        .into_iter()
        .filter_map(|(_, permission)| permission)
        .map(|perm| perm.name)
        .collect();

    for required_permission in required_permissions {
        if !role_permissions.contains(&required_permission.to_string()) {
            return Err(common_response(
                StatusCode::FORBIDDEN,
                "You don't have the required permissions",
            ));
        }
    }

    Ok(())
}
