use crate::TestsItemListDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionsTestDto {
	pub start_date: String,
	pub end_date: String,
	pub test_id: String,
	pub weight: String,
	pub multiplier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionsRequestUpdateDto {
	pub session_name: String,
	pub student_type: String,
	pub is_active: bool,
	pub description: String,
	pub tests: Vec<SessionsTestDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionsRequestCreateDto {
	pub session_name: String,
	pub student_type: Option<String>,
	pub is_active: bool,
	pub description: String,
	pub tests: Vec<SessionsTestDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionsItemListDto {
	pub id: String,
	pub session_name: String,
	pub student_type: Option<String>,
	pub is_active: bool,
	pub description: String,
	pub test_count: u64,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionsItemDto {
	pub id: String,
	pub session_name: String,
	pub student_type: Option<String>,
	pub is_active: bool,
	pub description: String,
	pub tests: Vec<TestsItemListDto>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}
