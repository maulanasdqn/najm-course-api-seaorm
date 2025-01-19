use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::apps::users::users_dto::UsersItemDto;

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AuthLoginDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthRegisterDto {
    pub fullname: String,
    pub email: String,
    pub student_type: String,
    pub phone_number: String,
    pub password: String,
    pub referral_code: Option<String>,
    pub referred_by: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AuthForgotDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AuthVerifyEmailDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub otp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AuthRequestNewPasswordDto {
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub old_password: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthTokenDto {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthRefreshDto {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthDataDto {
    pub token: AuthTokenDto,
    pub user: UsersItemDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    pub data: AuthDataDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}
