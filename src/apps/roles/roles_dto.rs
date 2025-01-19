use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{apps::permissions::permissions_dto::PermissionsItemDto, utils::meta::TMetaResponse};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesRequestDto {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesItemDto {
    pub id: String,
    pub name: String,
    pub permissions: Vec<PermissionsItemDto>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesListResponseDto {
    pub data: Vec<RolesItemDto>,
    pub meta: TMetaResponse,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RolesDetailResponseDto {
    pub data: RolesItemDto,
}
