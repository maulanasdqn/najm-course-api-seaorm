use axum::{
    body::Body,
    extract::Request,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Response, StatusCode,
    },
    middleware::Next,
};
use serde_json::json;
use std::convert::Infallible;

use crate::{decode_access_token, users::query_get_user_by_email};

pub fn format_error(message: String) -> Response<Body> {
    let error_body = json!({
        "message": message
    });

    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(error_body.to_string()))
        .unwrap()
}

pub async fn authorization_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, Infallible> {
    let auth_header = req.headers_mut().get(AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str(),
        None => {
            return Ok(format_error("You not authorized".to_string()));
        }
    };

    let auth_header = match auth_header {
        Ok(header) => header.to_string(),
        Err(_) => {
            return Ok(format_error("Invalid header".to_string()));
        }
    };

    let mut header_parts = auth_header.split_whitespace();

    let token = match header_parts.nth(1) {
        Some(token) => token,
        None => {
            return Ok(format_error("Invalid token".to_string()));
        }
    };

    let token_data = match decode_access_token(token.to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Ok(format_error("Invalid token".to_string()));
        }
    };

    let user_response = query_get_user_by_email(token_data.claims.email).await;
    if user_response.data.email.is_empty() {
        return Ok(format_error("Unauthorized user".to_string()));
    }

    req.extensions_mut().insert(user_response);

    let response = next.run(req).await;
    Ok(response)
}
