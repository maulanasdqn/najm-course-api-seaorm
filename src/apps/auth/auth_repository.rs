use crate::apps::auth::auth_dto::{AuthDataDto, AuthLoginDto, AuthResponse, AuthTokenDto};
use crate::apps::users::users_repository::query_get_user_by_email;
use crate::utils::jwt::{encode_access_token, encode_refresh_token};
use crate::utils::password::verify_password;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub async fn mutation_login(Json(credentials): Json<AuthLoginDto>) -> Response {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Email and password are required" })),
        )
            .into_response();
    }

    let user_response = query_get_user_by_email(credentials.email.clone()).await;

    if user_response.data.email.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Account not found" })),
        )
            .into_response();
    }

    let hashed_password = user_response.data.password.clone();

    let is_password_valid =
        verify_password(&credentials.password, &hashed_password).unwrap_or(false);

    if !is_password_valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Invalid credentials" })),
        )
            .into_response();
    }

    let access_token = encode_access_token(user_response.data.email.clone()).unwrap();
    let refresh_token = encode_refresh_token(user_response.data.email.clone()).unwrap();

    let auth_response = AuthResponse {
        data: AuthDataDto {
            token: AuthTokenDto {
                access_token,
                refresh_token,
            },
            user: user_response.data.clone(),
        },
    };

    (StatusCode::OK, Json(auth_response)).into_response()
}
