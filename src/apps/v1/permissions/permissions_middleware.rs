use axum::response::Response;
use hyper::{HeaderMap, StatusCode};
use redis::Commands;
use serde_json::from_str;

use crate::{common_response, connect_redis, decode_access_token};

use super::PermissionsEnum;
use crate::auth::AuthUsersItemDto;

pub async fn permissions_middleware(
    headers: &HeaderMap,
    required_permissions: Vec<PermissionsEnum>,
) -> Result<(), Response> {
    let mut redis = connect_redis();

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

    let token_data = match decode_access_token(token) {
        Ok(data) => data,
        Err(_) => {
            return Err(common_response(
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token",
            ));
        }
    };

    let email = token_data.claims.email;

    let redis_key = format!("authenticated_users_data:{}", email);

    let user_data: Option<String> = match redis.get(&redis_key) {
        Ok(Some(data)) => Some(data),
        Ok(None) => None,
        Err(_) => {
            return Err(common_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve user data from Redis",
            ));
        }
    };

    let user: AuthUsersItemDto = match user_data {
        Some(data) => match from_str(&data) {
            Ok(user) => user,
            Err(_) => {
                return Err(common_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to deserialize user data",
                ));
            }
        },
        None => {
            return Err(common_response(
                StatusCode::UNAUTHORIZED,
                "User session expired",
            ));
        }
    };

    let role = match user.role {
        Some(role) => role,
        None => {
            return Err(common_response(
                StatusCode::FORBIDDEN,
                "User does not have an assigned role",
            ));
        }
    };

    let role_permissions: Vec<String> = role
        .permissions
        .iter()
        .map(|perm| perm.name.clone())
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
