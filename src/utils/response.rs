use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

use crate::get_version;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ResponseSuccessDto<T: Serialize> {
    pub data: T,
}

#[derive(Serialize, Deserialize)]
struct ResponseErrorDto {
    status: String,
    message: String,
    version: String,
}

pub fn success_response<T: Serialize>(params: ResponseSuccessDto<T>) -> Response {
    let version = get_version().unwrap();
    (
        StatusCode::OK,
        Json(json!({
            "data": params.data,
            "version": version,
        })),
    )
        .into_response()
}

pub fn common_response(status: StatusCode, message: &str) -> Response {
    let version = get_version().unwrap();
    (
        status,
        Json(json!({
            "message": message,
            "version": version,
        })),
    )
        .into_response()
}
