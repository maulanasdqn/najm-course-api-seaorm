use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsRequestUpdateDto {
	pub test_id: String,
	pub question: String,
	pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsRequestCreateDto {
	pub test_id: String,
	pub question: String,
	pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsItemListDto {
	pub id: String,
	pub question: String,
	pub description: String,
	pub options: Vec<OptionsItemListDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsItemDto {
	pub id: String,
	pub question: String,
	pub description: String,
	pub options: Vec<OptionsItemListDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsRequestUpdateDto {
	pub label: Option<String>,
	pub question_id: String,
	pub is_correct: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsRequestCreateDto {
	pub label: String,
	pub question_id: String,
	pub is_correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsItemListDto {
	pub id: String,
	pub question_id: String,
	pub label: String,
	pub is_correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsItemDto {
	pub id: String,
	pub question_id: String,
	pub label: String,
	pub is_correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsRequestUpdateDto {
	pub test_name: Option<String>,
	pub session_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsRequestCreateDto {
	pub test_name: String,
	pub session_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsItemListDto {
	pub id: String,
	pub test_name: String,
	pub question_count: usize,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsItemDto {
	pub id: String,
	pub test_name: String,
	pub session_id: String,
	pub questions: Vec<QuestionsItemDto>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}
