use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::TestsItemListDto;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionsRequestUpdateDto {
	pub session_name: Option<String>,
	pub student_type: Option<String>,
	pub start_date: Option<String>,
	pub end_date: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionsRequestCreateDto {
	pub session_name: String,
	pub student_type: Option<String>,
	pub start_date: String,
	pub end_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionsItemListDto {
	pub id: String,
	pub session_name: String,
	pub student_type: Option<String>,
	pub start_date: String,
	pub end_date: String,
	pub test_count: u64,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionsItemDto {
	pub id: String,
	pub session_name: String,
	pub student_type: Option<String>,
	pub start_date: String,
	pub end_date: String,
	pub tests: Vec<TestsItemListDto>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}
