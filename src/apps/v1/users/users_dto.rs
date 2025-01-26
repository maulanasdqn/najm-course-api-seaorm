use crate::{apps::v1::roles::roles_dto::RolesItemDto, utils::dto::MetaResponseDto};
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
    pub role_id: String,
    pub fullname: String,
    pub email: String,
    pub student_type: String,
    pub phone_number: String,
    pub password: String,
    pub referral_code: Option<String>,
    pub referred_by: Option<String>,
    pub avatar: Option<String>,
    pub birthdate: Option<String>,
    pub gender: Option<String>,
    pub identity_number: Option<String>,
    pub religion: Option<String>,
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
    pub role: Option<RolesItemDto>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl UsersItemDto {
    pub(crate) fn default() -> UsersItemDto {
        UsersItemDto {
            id: String::new(),
            fullname: String::new(),
            email: String::new(),
            avatar: None,
            phone_number: String::new(),
            referral_code: None,
            referred_by: None,
            role: None,
            created_at: None,
            updated_at: None,
        }
    }
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
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersCheckLoginDto {
    pub password: String,
    pub is_active: bool,
}

impl UsersCheckLoginDto {
    pub(crate) fn default() -> UsersCheckLoginDto {
        UsersCheckLoginDto {
            password: String::new(),
            is_active: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UsersListResponseDto {
    pub data: Vec<UsersItemListDto>,
    pub meta: Option<MetaResponseDto>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UsersDetailResponseDto {
    pub data: UsersItemDto,
    pub version: String,
}
