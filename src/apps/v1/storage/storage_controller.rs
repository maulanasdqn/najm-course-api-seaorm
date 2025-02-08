use super::{mutation_upload_file, StorageRequestDto, StorageResponseDto};
use crate::{AppState, MessageResponseDto};
use axum::{
	extract::{Multipart, State},
	response::Response,
};

#[utoipa::path(
    post,
    path = "/v1/storage/upload",
    request_body(
        content= StorageRequestDto,
        content_type = "multipart/form-data"
    ),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "File Uploaded", body = StorageResponseDto),
        (status = 400, description = "Failed to upload file", body = MessageResponseDto)
    ),
    tag = "Storage"
)]
pub async fn post_upload(
	State(state): State<AppState>,
	payload: Multipart,
) -> Response {
	mutation_upload_file(State(state), payload).await
}
