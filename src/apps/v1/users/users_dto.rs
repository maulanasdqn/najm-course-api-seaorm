use crate::roles::RolesItemDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UsersCreateRequestDto {
	pub role_id: String,
	pub fullname: String,
	pub email: String,
	pub student_type: String,
	pub phone_number: String,
	pub password: String,
	pub referral_code: Option<String>,
	pub referred_by: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersUpdateRequestDto {
	pub role_id: Option<String>,
	pub fullname: Option<String>,
	pub email: Option<String>,
	pub student_type: Option<String>,
	pub phone_number: Option<String>,
	pub avatar: Option<String>,
	pub birthdate: Option<String>,
	pub gender: Option<String>,
	pub identity_number: Option<String>,
	pub religion: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersActiveInactiveRequestDto {
	pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersItemDto {
	pub id: String,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub referral_code: Option<String>,
	pub referred_by: Option<String>,
	pub identity_number: Option<String>,
	pub is_active: bool,
	pub student_type: String,
	pub religion: Option<String>,
	pub gender: Option<String>,
	pub role: Option<RolesItemDto>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersItemListDto {
	pub id: String,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub referral_code: Option<String>,
	pub referred_by: Option<String>,
	pub role: String,
	pub is_active: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersCheckLoginDto {
	pub password: String,
	pub is_active: bool,
}
