use axum::{extract::Query, response::IntoResponse};

use crate::apps::users::users_dto::TMetas;

use super::users_repository::query_get_users;

#[utoipa::path(
    get,
    path = "/api/users",
    params(TMetas),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "List Users", body = ReservationListResponse),
        (status = 400, description = "Invalid Users data", body = MessageResponse)
    ),
    tag = "Reservations"
)]

pub async fn get_users(Query(params): Query<TMetas>) -> impl IntoResponse {
    query_get_users(params).await
}
