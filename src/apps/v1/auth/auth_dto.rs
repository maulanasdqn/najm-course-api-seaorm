use crate::roles::RolesItemDto;
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
	pub otp: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthNewPasswordRequestDto {
	pub password: String,
	pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthChangePasswordRequestDto {
	pub password: String,
	pub old_password: String,
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
	pub is_profile_completed: bool,
	pub role: Option<RolesItemDto>,
}
