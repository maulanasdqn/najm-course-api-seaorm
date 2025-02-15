use crate::{MessageResponseDto, ResponseSuccessDto};

use super::{
	mutation_change_password, mutation_forgot_password, mutation_login,
	mutation_new_password, mutation_refresh, mutation_register, mutation_send_otp,
	mutation_verify_email, AuthChangePasswordRequestDto, AuthDataDto,
	AuthForgotRequestDto, AuthLoginRequestDto, AuthNewPasswordRequestDto,
	AuthRefreshTokenRequestDto, AuthRegisterRequestDto, AuthTokenItemDto,
	AuthVerifyEmailRequestDto,
};

use axum::{extract::Json, response::IntoResponse};
use hyper::HeaderMap;

#[utoipa::path(
    post,
    path = "/v1/auth/login",
    request_body = AuthLoginRequestDto,
    responses(
        (status = 200, description = "Login successful", body = ResponseSuccessDto<AuthDataDto>),
        (status = 401, description = "Unauthorized", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]

pub async fn post_login(
	Json(payload): Json<AuthLoginRequestDto>,
) -> impl IntoResponse {
	mutation_login(Json(payload)).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/register",
    request_body = AuthRegisterRequestDto,
    responses(
        (status = 200, description = "Register successful", body = MessageResponseDto),
        (status = 401, description = "Unauthorized", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]

pub async fn post_register(
	Json(payload): Json<AuthRegisterRequestDto>,
) -> impl IntoResponse {
	mutation_register(Json(payload)).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/forgot",
    request_body = AuthForgotRequestDto,
    responses(
        (status = 200, description = "Forgot password successful", body = MessageResponseDto),
        (status = 401, description = "Unauthorized", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]

pub async fn post_forgot(
	Json(payload): Json<AuthForgotRequestDto>,
) -> impl IntoResponse {
	mutation_forgot_password(Json(payload)).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/send-otp",
    request_body = AuthForgotRequestDto,
    responses(
        (status = 200, description = "Send OTP successful", body = MessageResponseDto),
        (status = 401, description = "Unauthorized", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]

pub async fn post_send_otp(
	Json(payload): Json<AuthForgotRequestDto>,
) -> impl IntoResponse {
	mutation_send_otp(Json(payload)).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/new-password",
    request_body = AuthNewPasswordRequestDto,
    responses(
        (status = 200, description = "Reset password successful", body = MessageResponseDto),
        (status = 401, description = "Unauthorized", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]

pub async fn post_new_password(
	Json(payload): Json<AuthNewPasswordRequestDto>,
) -> impl IntoResponse {
	mutation_new_password(Json(payload)).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/verify-email",
    request_body = AuthVerifyEmailRequestDto,
    responses(
        (status = 200, description = "Verify email successful", body = MessageResponseDto),
        (status = 401, description = "Unauthorized", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]

pub async fn post_verify_email(
	Json(payload): Json<AuthVerifyEmailRequestDto>,
) -> impl IntoResponse {
	mutation_verify_email(Json(payload)).await
}

#[utoipa::path(
    post,
    path = "/v1/auth/refresh",
    request_body = AuthRefreshTokenRequestDto,
    responses(
        (status = 200, description = "Refresh successful", body = ResponseSuccessDto<AuthTokenItemDto>),
        (status = 401, description = "Unauthorized", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]

pub async fn post_refresh(
	Json(payload): Json<AuthRefreshTokenRequestDto>,
) -> impl IntoResponse {
	mutation_refresh(Json(payload)).await
}

#[utoipa::path(
    post,
    path = "/v1/users/change-password",
    security(
        ("Bearer" = [])
    ),
    request_body = AuthChangePasswordRequestDto,
    responses(
        (status = 200, description = "Change Password successful", body = MessageResponseDto),
        (status = 401, description = "Unauthorized", body = MessageResponseDto)
    ),
    tag = "Authentication"
)]

pub async fn post_change_password(
	headers: HeaderMap,
	Json(payload): Json<AuthChangePasswordRequestDto>,
) -> impl IntoResponse {
	mutation_change_password(headers, Json(payload)).await
}
