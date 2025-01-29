use crate::{common_response, roles::RolesItemDto};
use axum::response::Response;
use email_address::EmailAddress;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthLoginRequestDto {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Email(String);

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Password(String);

impl Email {
    pub fn parse(email: &String) -> Result<Email, Response> {
        match email.is_empty() {
            true => Err(common_response(
                StatusCode::BAD_REQUEST,
                "Email cannot be empty",
            )),
            false => match EmailAddress::is_valid(&email) {
                true => Ok(Email(email.to_string())),
                false => Err(common_response(
                    StatusCode::BAD_REQUEST,
                    "Email must be valid",
                )),
            },
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Password {
    pub fn parse(password: &String) -> Result<Password, Response> {
        match password.len() {
            0 => Err(common_response(
                StatusCode::BAD_REQUEST,
                "Password cannot be empty",
            )),
            1..=7 => Err(common_response(
                StatusCode::BAD_REQUEST,
                "Password cannot be empty",
            )),
            8..=64 => {
                let has_uppercase = password.chars().any(|c| c.is_uppercase());
                let has_lowercase = password.chars().any(|c| c.is_lowercase());
                let has_digit = password.chars().any(|c| c.is_ascii_digit());
                match (has_uppercase, has_lowercase, has_digit) {
                    (true, true, true) => Ok(Password(password.to_string())),
                    _ => Err(common_response(
                        StatusCode::BAD_REQUEST,
                        "Password must contain at least one uppercase letter, one lowercase letter, and one number",
                    )),
                }
            }
            _ => Err(common_response(
                StatusCode::BAD_REQUEST,
                "Password must be at most 64 characters long",
            )),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
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

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthForgotRequestDto {
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthVerifyEmailRequestDto {
    pub email: String,
    pub otp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthNewPasswordRequestDto {
    pub password: String,
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
    pub user: AuthUsersItemDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthUsersItemDto {
    pub id: String,
    pub email: String,
    pub fullname: String,
    pub avatar: Option<String>,
    pub phone_number: String,
    pub role: Option<RolesItemDto>,
}
