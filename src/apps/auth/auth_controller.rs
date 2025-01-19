use super::{
    auth_dto::{AuthLoginDto, AuthRegisterDto},
    auth_repository::{mutation_login, mutation_register},
};
use axum::{extract::Json, response::IntoResponse};

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = AuthLoginDto,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Unauthorized", body = MessageResponse)
    ),
    tag = "Authentication"
)]

pub async fn post_login(Json(payload): Json<AuthLoginDto>) -> impl IntoResponse {
    mutation_login(Json(payload)).await
}

#[utoipa::path(
    post,
    path = "/api/auth/forgot",
    request_body = AuthForgotDto,
    responses(
        (status = 200, description = "Forgot password successful", body = MessageResponse),
        (status = 401, description = "Unauthorized", body = MessageResponse)
    ),
    tag = "Authentication"
)]

pub async fn post_forgot() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    post,
    path = "/api/auth/send-otp",
    request_body = AuthForgotDto,
    responses(
        (status = 200, description = "Send OTP successful", body = MessageResponse),
        (status = 401, description = "Unauthorized", body = MessageResponse)
    ),
    tag = "Authentication"
)]

pub async fn post_send_otp() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    post,
    path = "/api/auth/new-password",
    request_body = AuthRequestNewPasswordDto,
    responses(
        (status = 200, description = "Reset password successful", body = MessageResponse),
        (status = 401, description = "Unauthorized", body = MessageResponse)
    ),
    tag = "Authentication"
)]

pub async fn post_new_password() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    post,
    path = "/api/auth/verify-email",
    request_body = AuthVerifyEmailDto,
    responses(
        (status = 200, description = "Verify email successful", body = MessageResponse),
        (status = 401, description = "Unauthorized", body = MessageResponse)
    ),
    tag = "Authentication"
)]

pub async fn post_verify_email() -> impl IntoResponse {
    ()
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = AuthRegisterDto,
    responses(
        (status = 200, description = "Register successful", body = MessageResponse),
        (status = 401, description = "Unauthorized", body = MessageResponse)
    ),
    tag = "Authentication"
)]

pub async fn post_register(Json(payload): Json<AuthRegisterDto>) -> impl IntoResponse {
    mutation_register(Json(payload)).await
}
