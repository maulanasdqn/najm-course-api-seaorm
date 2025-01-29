use axum::{
    body::Body,
    extract::Request,
    http::{header::AUTHORIZATION, Response, StatusCode},
    middleware::Next,
};
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait};
use std::convert::Infallible;

use crate::{
    common_response, decode_access_token, get_db,
    schemas::{UsersColumn, UsersEntity},
};

pub async fn authorization_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, Infallible> {
    let db = get_db().await;

    let auth_header = req.headers_mut().get(AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str(),
        None => {
            return Ok(common_response(
                StatusCode::UNAUTHORIZED,
                "You are not authorized",
            ));
        }
    };

    let auth_header = match auth_header {
        Ok(header) => header.to_string(),
        Err(_) => {
            return Ok(common_response(StatusCode::UNAUTHORIZED, "Invalid header"));
        }
    };

    let mut header_parts = auth_header.split_whitespace();

    let token = match header_parts.nth(1) {
        Some(token) => token,
        None => {
            return Ok(common_response(StatusCode::UNAUTHORIZED, "Invalid token"));
        }
    };

    let token_data = match decode_access_token(token.to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Ok(common_response(
                StatusCode::UNAUTHORIZED,
                "Invalid token or expired",
            ));
        }
    };

    let user = UsersEntity::find()
        .filter(UsersColumn::Email.eq(token_data.claims.email.clone()))
        .one(&db)
        .await;

    match user {
        Ok(Some(user)) => {
            req.extensions_mut().insert(user);
            let response = next.run(req).await;
            Ok(response)
        }
        Ok(None) => Ok(common_response(
            StatusCode::UNAUTHORIZED,
            "Unauthorized user",
        )),
        Err(err) => Ok(common_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            &err.to_string(),
        )),
    }
}
