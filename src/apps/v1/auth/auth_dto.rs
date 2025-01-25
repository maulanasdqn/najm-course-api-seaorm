use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::apps::v1::users::users_dto::UsersItemDto;

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AuthLoginRequestDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthRegisterRequestDto {
    pub fullname: String,
    pub email: String,
    pub student_type: String,
    pub phone_number: String,
    pub password: String,
    pub referral_code: Option<String>,
    pub referred_by: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AuthForgotRequestDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AuthVerifyEmailRequestDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub otp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AuthNewPasswordRequestDto {
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthTokenItemDto {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthRefreshTokenRequestDto {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthDataDto {
    pub token: AuthTokenItemDto,
    pub user: UsersItemDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthResponseDto {
    pub data: AuthDataDto,
    pub version: String,
}
