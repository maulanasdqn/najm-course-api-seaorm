use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersCreateDto {
    pub fullname: String,
    pub email: String,
    pub avatar: Option<String>,
    pub phone_number: String,
    pub password: String,
    pub referral_code: Option<String>,
    pub referred_by: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub role: Option<RolesDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesDto {
    pub id: String,
    pub name: String,
    pub permissions: Vec<PermissionsDto>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PermissionsDto {
    pub id: String,
    pub name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UsersListResponse {
    pub data: Vec<UsersItemDto>,
    pub meta: Option<TMetas>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UsersDetailResponse {
    pub data: UsersItemDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct TMetas {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub total: Option<u64>,
}
