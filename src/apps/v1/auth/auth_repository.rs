use crate::apps::v1::roles::roles_dto::RolesItemDto;
use crate::apps::v1::users::users_dto::UsersCheckLoginDto;
use crate::apps::v1::users::users_repository::query_get_user_by_id;
use crate::get_version;
use crate::libs::database::get_db;
use crate::libs::database::schemas::app_roles_schema::{Column as RoleColumn, Entity as Role};
use crate::libs::database::schemas::app_users_schema::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as User,
};
use crate::libs::email::send_email;
use crate::libs::otp::OtpManager;
use crate::utils::error::AppError;
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

use super::auth_dto::{
    AuthDataDto, AuthForgotRequestDto, AuthLoginRequestDto, AuthRegisterRequestDto,
    AuthResponseDto, AuthTokenItemDto, AuthVerifyEmailRequestDto,
};

pub async fn query_get_user_by_email(email: String) -> Result<Json<UsersCheckLoginDto>, AppError> {
    let db: DatabaseConnection = get_db().await;

    if let Some(user) = User::find()
        .filter(UserColumn::Email.eq(email))
        .one(&db)
        .await?
    {
        let user_detail = UsersCheckLoginDto {
            id: user.id.to_string(),
            fullname: user.fullname,
            email: user.email,
            password: user.password,
            is_active: user.is_active,
        };

        Ok(Json(user_detail))
    } else {
        Err(AppError::NotFound)
    }
}

async fn query_get_role_student_id(db: &DatabaseConnection) -> Result<RolesItemDto, AppError> {
    Role::find()
        .filter(RoleColumn::Name.eq("Student"))
        .one(db)
        .await
        .map_err(|err| AppError::DatabaseError(err))?
        .map(|r| RolesItemDto {
            id: r.id.to_string(),
            name: r.name,
            permissions: vec![],
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        })
        .ok_or(AppError::NotFound)
}

pub async fn mutation_login(Json(credentials): Json<AuthLoginRequestDto>) -> Response {
    let version = get_version().unwrap();
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Email and password are required" })),
        )
            .into_response();
    }

    let user_response = query_get_user_by_email(credentials.email.clone())
        .await
        .unwrap();

    if user_response.email.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Account not found" })),
        )
            .into_response();
    }

    let hashed_password = &user_response.password;
    let is_active = &user_response.is_active;

    let is_password_valid =
        verify_password(&credentials.password, &hashed_password).unwrap_or(false);

    if !is_password_valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Invalid credentials" })),
        )
            .into_response();
    }

    if !is_active {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Account is not active" })),
        )
            .into_response();
    }

    let access_token = encode_access_token(user_response.email.clone()).unwrap();
    let refresh_token = encode_refresh_token(user_response.email.clone()).unwrap();
    let user_data = query_get_user_by_id(Uuid::parse_str(user_response.id.as_str()).unwrap())
        .await
        .unwrap();

    let auth_response = AuthResponseDto {
        data: AuthDataDto {
            token: AuthTokenItemDto {
                access_token,
                refresh_token,
            },
            user: user_data.data.clone(),
        },
        version,
    };

    (StatusCode::OK, Json(auth_response)).into_response()
}

pub async fn mutation_register(new_user: Json<AuthRegisterRequestDto>) -> Response {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    let existing_user = User::find()
        .filter(UserColumn::Email.eq(new_user.email.clone()))
        .one(&db)
        .await;

    if let Ok(Some(_)) = existing_user {
        return (
            StatusCode::CONFLICT,
            Json(json!({ "message": "User with this email already exists", "version": version })),
        )
            .into_response();
    }

    if new_user.password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Password must be at least 8 characters long", "version": version })),
        )
            .into_response();
    }

    let hashed_password = match hash_password(&new_user.password) {
        Ok(hash) => hash,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Email or password is invalid", "version": version })),
            )
                .into_response();
        }
    };

    let otp_manager = OtpManager::new(300);
    let otp = otp_manager.generate_otp(&new_user.email);

    let active_model = UserActiveModel {
        id: Set(Uuid::new_v4()),
        role_id: Set(
            Uuid::parse_str(query_get_role_student_id(&db).await.unwrap().id.as_str())
                .unwrap_or(Uuid::new_v4()),
        ),
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

    send_email(
        &new_user.email,
        "Verification",
        &format!("Your OTP Code is {}", otp),
    )
    .unwrap();

    match active_model.insert(&db).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "User created successfully", "version": version })),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to create user", "version": version, "error": err.to_string() })),
        )
            .into_response(),
    }
}

pub async fn mutation_forgot_password(Json(payload): Json<AuthForgotRequestDto>) -> Response {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    if payload.email.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Email is required", "version": version })),
        )
            .into_response();
    }

    let user = User::find()
        .filter(UserColumn::Email.eq(payload.email.clone()))
        .one(&db)
        .await;

    if let Ok(Some(user)) = user {
        let reset_token = encode_access_token(user.email.clone()).unwrap();
        send_email(&user.email, "Reset Password", &reset_token).unwrap();

        return (
            StatusCode::OK,
            Json(json!({ "message": "Password reset token sent", "version": version, "reset_token": reset_token })),
        )
            .into_response();
    }

    (
        StatusCode::NOT_FOUND,
        Json(json!({ "message": "User not found", "version": version })),
    )
        .into_response()
}

pub async fn mutation_send_otp(Json(payload): Json<AuthForgotRequestDto>) -> Response {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    if payload.email.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Email is required", "version": version })),
        )
            .into_response();
    }

    let user = User::find()
        .filter(UserColumn::Email.eq(payload.email.clone()))
        .one(&db)
        .await;

    if let Ok(Some(user)) = user {
        let otp_manager = OtpManager::new(300);
        let otp = otp_manager.generate_otp(&user.email);

        send_email(
            &user.email,
            "Verification",
            &format!("Your OTP Code is {}", otp),
        )
        .unwrap();

        return (
            StatusCode::OK,
            Json(json!({ "message": "OTP Has Been sent", "version": version,})),
        )
            .into_response();
    }

    (
        StatusCode::NOT_FOUND,
        Json(json!({ "message": "User not found", "version": version })),
    )
        .into_response()
}

pub async fn mutation_verify_email(Json(payload): Json<AuthVerifyEmailRequestDto>) -> Response {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    if payload.email.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Email is required", "version": version })),
        )
            .into_response();
    }

    let otp_manager = OtpManager::new(300);
    let is_valid = otp_manager.validate_otp(&payload.email, &payload.otp);

    if is_valid {
        if let Some(user) = User::find()
            .filter(UserColumn::Email.eq(payload.email.clone()))
            .one(&db)
            .await
            .unwrap()
        {
            let mut active_user: UserActiveModel = user.into();
            active_user.is_active = Set(true);

            if let Err(err) = active_user.update(&db).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": "Failed to update user status",
                        "version": version,
                        "error": err.to_string()
                    })),
                )
                    .into_response();
            }

            return (
                StatusCode::OK,
                Json(json!({ "message": "Email successfully verified", "version": version })),
            )
                .into_response();
        }
    }

    (
        StatusCode::BAD_REQUEST,
        Json(json!({ "message": "Invalid OTP", "version": version })),
    )
        .into_response()
}
