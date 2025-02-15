use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestsItemDto {
	pub id: String,
	pub name: String,
	pub description: Option<String>,
	pub instructions: Option<String>,
	pub time_limit: Option<i32>,
	pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestsItemListDto {
	pub id: String,
	pub name: String,
	pub description: Option<String>,
	pub instructions: Option<String>,
	pub time_limit: Option<i32>,
	pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestsRequestCreateDto {
	pub name: String,
	pub description: Option<String>,
	pub instructions: Option<String>,
	pub time_limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestsRequestUpdateDto {
	pub name: Option<String>,
	pub description: Option<String>,
	pub instructions: Option<String>,
	pub time_limit: Option<i32>,
}
