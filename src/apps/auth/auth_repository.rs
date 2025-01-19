use crate::apps::auth::auth_dto::{AuthDataDto, AuthLoginDto, AuthResponse, AuthTokenDto};
use crate::apps::users::users_repository::query_get_user_by_email;
use crate::libs::database::get_db;
use crate::libs::database::schemas::app_users_schema::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as User,
};
use crate::utils::jwt::{encode_access_token, encode_refresh_token};
use crate::utils::password::{hash_password, verify_password};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;

use super::auth_dto::AuthRegisterDto;

pub async fn mutation_login(Json(credentials): Json<AuthLoginDto>) -> Response {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Email and password are required" })),
        )
            .into_response();
    }

    let user_response = query_get_user_by_email(credentials.email.clone()).await;

    if user_response.data.email.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Account not found" })),
        )
            .into_response();
    }

    let hashed_password = user_response.data.password.clone();

    let is_password_valid =
        verify_password(&credentials.password, &hashed_password).unwrap_or(false);

    if !is_password_valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Invalid credentials" })),
        )
            .into_response();
    }

    let access_token = encode_access_token(user_response.data.email.clone()).unwrap();
    let refresh_token = encode_refresh_token(user_response.data.email.clone()).unwrap();

    let auth_response = AuthResponse {
        data: AuthDataDto {
            token: AuthTokenDto {
                access_token,
                refresh_token,
            },
            user: user_response.data.clone(),
        },
    };

    (StatusCode::OK, Json(auth_response)).into_response()
}

pub async fn mutation_register(new_user: Json<AuthRegisterDto>) -> Response {
    let db: DatabaseConnection = get_db().await;

    let existing_user = User::find()
        .filter(UserColumn::Email.eq(new_user.email.clone()))
        .one(&db)
        .await;

    if let Ok(Some(_)) = existing_user {
        return (
            StatusCode::CONFLICT,
            Json(json!({ "message": "User with this email already exists" })),
        )
            .into_response();
    }

    if new_user.password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Password must be at least 8 characters long" })),
        )
            .into_response();
    }

    let hashed_password = match hash_password(&new_user.password) {
        Ok(hash) => hash,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to hash password" })),
            )
                .into_response();
        }
    };

    let active_model = UserActiveModel {
        id: Set(Uuid::new_v4()),
        role_id: Set(Uuid::new_v4()),
        fullname: Set(new_user.fullname.clone()),
        email: Set(new_user.email.clone()),
        email_verified: Set(Some(Utc::now())),
        avatar: Set(Some("".to_string())),
        phone_number: Set(new_user.phone_number.clone()),
        password: Set(hashed_password),
        referral_code: Set(new_user.referral_code.clone()),
        referred_by: Set(new_user.referred_by.clone()),
        birth_date: Set(Some(Utc::now())),
        gender: Set(Some("".to_string())),
        religion: Set(Some("".to_string())),
        identity_number: Set(Some("".to_string())),
        is_deleted: Set(false),
        is_active: Set(false),
        is_profile_completed: Set(false),
        student_type: Set(new_user.student_type.clone()),
        created_at: Set(Some(Utc::now())),
        updated_at: Set(Some(Utc::now())),
    };

    match active_model.insert(&db).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "User created successfully" })),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to create user", "error": err.to_string() })),
        )
            .into_response(),
    }
}
