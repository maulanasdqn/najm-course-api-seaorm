use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    apps::v1::permissions::permissions_dto::PermissionsItemDto, utils::dto::MetaResponseDto,
};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesRequestDto {
    pub name: String,
    pub permissions: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesItemDto {
    pub id: String,
    pub name: String,
    pub permissions: Vec<PermissionsItemDto>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolesIdOnlyDto {
    pub id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesListResponseDto {
    pub data: Vec<RolesItemDto>,
    pub meta: MetaResponseDto,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesDetailResponseDto {
    pub data: RolesItemDto,
    pub version: String,
}
