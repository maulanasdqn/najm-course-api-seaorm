use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TestAnswersItemDto {
	pub id: String,
	pub user_id: String,
	pub test_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct QuestionsAnswersRequestCreateDto {
	pub question_id: String,
	pub option_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TestAnswersRequestCreateDto {
	pub test_id: String,
	pub questions: Vec<QuestionsAnswersRequestCreateDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsItemDto {
	pub id: String,
	pub question: String,
	pub discussion: String,
	pub options: Vec<OptionsItemDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsItemDto {
	pub id: String,
	pub label: String,
	pub is_correct: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsRequestCreateDto {
	pub question: String,
	pub discussion: String,
	pub options: Vec<OptionsRequestCreateDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsRequestCreateDto {
	pub label: String,
	pub is_correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsRequestUpdateDto {
	pub test_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsRequestCreateDto {
	pub test_name: String,
	pub questions: Vec<QuestionsRequestCreateDto>,
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
	pub questions: Vec<QuestionsItemDto>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}
