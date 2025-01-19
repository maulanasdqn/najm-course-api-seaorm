use crate::{apps::roles::roles_dto::RolesItemDto, utils::meta::TMetaResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersRequestDto {
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
pub struct UsersItemDto {
    pub id: String,
    pub fullname: String,
    pub password: String,
    pub email: String,
    pub avatar: Option<String>,
    pub phone_number: String,
    pub referral_code: Option<String>,
    pub referred_by: Option<String>,
    pub role: Option<RolesItemDto>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UsersListResponseDto {
    pub data: Vec<UsersItemDto>,
    pub meta: Option<TMetaResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UsersDetailResponseDto {
    pub data: UsersItemDto,
}
