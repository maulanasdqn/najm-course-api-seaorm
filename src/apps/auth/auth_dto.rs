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
