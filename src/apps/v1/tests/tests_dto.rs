use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct IndividualQuestionsRequestCreateDto {
	pub test_id: String,
	pub question: String,
	pub discussion: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct IndividualOptionsRequestCreateDto {
	pub label: String,
	pub question_id: String,
	pub is_correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsItemDto {
	pub label: String,
	pub is_correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsItemDto {
	pub question: String,
	pub discussion: String,
	pub options: Vec<OptionsItemDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsRequestUpdateDto {
	pub test_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsRequestCreateDto {
	pub test_name: String,
	pub session_id: String,
	pub questions: Vec<QuestionsItemDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsItemListDto {
	pub id: String,
	pub test_name: String,
	pub question_count: u64,
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
