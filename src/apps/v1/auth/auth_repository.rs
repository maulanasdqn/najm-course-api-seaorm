use super::auth_dto::{
    AuthDataDto, AuthForgotRequestDto, AuthLoginRequestDto, AuthNewPasswordRequestDto,
    AuthRefreshTokenRequestDto, AuthRegisterRequestDto, AuthTokenItemDto,
    AuthVerifyEmailRequestDto,
};
use crate::{
    common_response, connect_redis, decode_access_token, decode_refresh_token, encode_access_token,
    encode_refresh_token, get_db, hash_password,
    roles::query_get_role_student_id,
    schemas::{UsersActiveModel, UsersColumn, UsersEntity},
    send_email, success_response,
    users::{query_get_user_by_email, query_password_and_is_active},
    verify_password, OtpManager, ResponseSuccessDto,
};
use axum::{http::StatusCode, response::Response, Json};
use chrono::Utc;
use redis::Commands;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set,
};
use std::env;
use uuid::Uuid;

pub async fn mutation_login(Json(credentials): Json<AuthLoginRequestDto>) -> Response {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return common_response(StatusCode::BAD_REQUEST, "Email and password are required");
    }

    let user_response = query_get_user_by_email(credentials.email.clone())
        .await
        .unwrap();

    if user_response.data.id.eq(&"".to_string()) || user_response.data.id.is_empty() {
        return common_response(StatusCode::UNAUTHORIZED, "Email or password invalid");
    }

    if user_response.data.email.is_empty() {
        return common_response(StatusCode::UNAUTHORIZED, "Email or password invalid");
    }

    let user_verify = query_password_and_is_active(user_response.data.email.clone())
        .await
        .unwrap();

    let hashed_password = &user_verify.password;

    let is_active = &user_verify.is_active;

    let is_password_valid =
        verify_password(&credentials.password, &hashed_password).unwrap_or(false);

    if !is_password_valid {
        return common_response(StatusCode::UNAUTHORIZED, "Email or password invalid");
    }

    if !is_active {
        return common_response(
            StatusCode::UNAUTHORIZED,
            "Your account is not active, please verify your email",
        );
    }

    let access_token = encode_access_token(credentials.email.clone()).unwrap();
    let refresh_token = encode_refresh_token(credentials.email.clone()).unwrap();

    let response = ResponseSuccessDto {
        data: AuthDataDto {
            token: AuthTokenItemDto {
                access_token,
                refresh_token,
            },
            user: user_response.data.clone(),
        },
    };

    success_response(response)
}

pub async fn mutation_register(new_user: Json<AuthRegisterRequestDto>) -> Response {
    let redis = connect_redis();

    let db: DatabaseConnection = get_db().await;

    let otp_manager = OtpManager::new(300);

    if let Ok(Some(_)) = UsersEntity::find()
        .filter(UsersColumn::Email.eq(new_user.email.clone()))
        .one(&db)
        .await
    {
        return common_response(StatusCode::CONFLICT, "User with that email already exist");
    }

    if new_user.password.len() < 8 {
        return common_response(
            StatusCode::BAD_REQUEST,
            "Password must be have 8 character long",
        );
    }

    let hashed_password = hash_password(&new_user.password).unwrap();

    let otp = otp_manager.generate_otp(redis, &new_user.email);

    let student_role = query_get_role_student_id().await.unwrap();

    let active_model = UsersActiveModel {
        id: Set(Uuid::new_v4()),
        role_id: Set(student_role.id),
        fullname: Set(new_user.fullname.clone()),
        email: Set(new_user.email.clone()),
        email_verified: Set(None),
        avatar: Set(None),
        phone_number: Set(new_user.phone_number.clone()),
        password: Set(hashed_password),
        referral_code: Set(new_user.referral_code.clone()),
        referred_by: Set(new_user.referred_by.clone()),
        birth_date: Set(None),
        gender: Set(None),
        religion: Set(None),
        identity_number: Set(None),
        is_deleted: Set(false),
        is_active: Set(false),
        is_profile_completed: Set(false),
        student_type: Set(new_user.student_type.clone()),
        created_at: Set(Some(Utc::now())),
        updated_at: Set(Some(Utc::now())),
    };

    let email_content = &format!("Your OTP Code is {}", otp);

    send_email(&new_user.email, "Verification", email_content).unwrap();

    match active_model.insert(&db).await {
        Ok(_) => common_response(StatusCode::CREATED, "User created successfully"),
        Err(err) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    }
}

pub async fn mutation_forgot_password(Json(payload): Json<AuthForgotRequestDto>) -> Response {
    let db: DatabaseConnection = get_db().await;

    if payload.email.is_empty() {
        return common_response(StatusCode::BAD_REQUEST, "Email is required");
    }

    let user = UsersEntity::find()
        .filter(UsersColumn::Email.eq(payload.email.clone()))
        .one(&db)
        .await;

    if let Ok(Some(user)) = user {
        let mut redis = connect_redis();

        let reset_token = encode_access_token(user.email.clone()).unwrap();

        let redis_key = format!("reset_password:{}", user.email);

        if let Err(err) = redis.set_ex::<_, _, ()>(
            &redis_key,
            reset_token.clone(),
            (3600 * 24).try_into().unwrap_or(86400),
        ) {
            return common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string());
        }

        let fe_url = env::var("FE_URL").unwrap_or("".to_string());
        let email_content = format!(
            "You have requested a password reset. Please click the link below to continue: {}/auth/reset-password?token={}",
            fe_url, reset_token
        );

        send_email(&user.email, "Reset Password Request", &email_content).unwrap();

        return common_response(StatusCode::OK, "Password reset token sent");
    }

    common_response(StatusCode::NOT_FOUND, "User not found")
}

pub async fn mutation_send_otp(Json(payload): Json<AuthForgotRequestDto>) -> Response {
    let db: DatabaseConnection = get_db().await;

    if payload.email.is_empty() {
        return common_response(StatusCode::BAD_REQUEST, "Email is required");
    }

    let user = UsersEntity::find()
        .filter(UsersColumn::Email.eq(payload.email.clone()))
        .one(&db)
        .await;

    if let Ok(Some(user)) = user {
        let redis = connect_redis();
        let otp_manager = OtpManager::new(300);
        let otp = otp_manager.generate_otp(redis, &user.email);
        let email_message = &format!("Your OTP Code is {}", otp);
        send_email(&user.email, "Verification", email_message).unwrap();
        return common_response(StatusCode::OK, "OTP Has Been sent");
    }

    common_response(StatusCode::NOT_FOUND, "User not found")
}

pub async fn mutation_verify_email(Json(payload): Json<AuthVerifyEmailRequestDto>) -> Response {
    let db: DatabaseConnection = get_db().await;
    let redis = connect_redis();
    let otp_manager = OtpManager::new(300);

    if payload.email.is_empty() {
        return common_response(StatusCode::BAD_REQUEST, "Email is required");
    }

    let is_valid = otp_manager.validate_otp(redis, &payload.email, &payload.otp);

    if is_valid {
        if let Some(user) = UsersEntity::find()
            .filter(UsersColumn::Email.eq(payload.email.clone()))
            .one(&db)
            .await
            .unwrap()
        {
            let mut active_user: UsersActiveModel = user.into();
            active_user.is_active = Set(true);
            active_user.email_verified = Set(Some(Utc::now()));

            if let Err(err) = active_user.update(&db).await {
                return common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string());
            }

            return common_response(StatusCode::OK, "Email successfully verified");
        }
    }

    common_response(StatusCode::BAD_REQUEST, "Invalid OTP")
}

pub async fn mutation_new_password(Json(payload): Json<AuthNewPasswordRequestDto>) -> Response {
    let db: DatabaseConnection = get_db().await;
    let mut redis = connect_redis();

    if payload.token.is_empty() || payload.password.is_empty() {
        return common_response(StatusCode::BAD_REQUEST, "Token and password are required");
    }

    if payload.password.len() < 8 {
        return common_response(
            StatusCode::BAD_REQUEST,
            "Password must be at least 8 characters long",
        );
    }

    let tok = decode_access_token(payload.token.clone());

    let email = tok.unwrap().claims.email;
    let key = format!("reset_password:{}", email);

    let stored_token: Option<String> = redis.get(&key).ok();

    if stored_token.as_deref() != Some(&payload.token) {
        return common_response(StatusCode::BAD_REQUEST, "Invalid or expired reset token");
    }

    let hashed_password = hash_password(&payload.password).unwrap();

    if let Some(user) = UsersEntity::find()
        .select_only()
        .column(UsersColumn::Password)
        .filter(UsersColumn::Email.eq(email.clone()))
        .one(&db)
        .await
        .ok()
        .flatten()
    {
        let mut active_user: UsersActiveModel = user.into();
        active_user.password = Set(hashed_password);

        if let Err(err) = active_user.update(&db).await {
            return common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string());
        }

        let _: () = redis.del(&key).unwrap_or(());

        return common_response(StatusCode::OK, "Password updated successfully");
    }

    common_response(StatusCode::NOT_FOUND, "User not found")
}

pub async fn mutation_refresh(Json(payload): Json<AuthRefreshTokenRequestDto>) -> Response {
    if payload.refresh_token.is_empty() {
        return common_response(StatusCode::BAD_REQUEST, "Refresh token is required");
    }

    let token_data = match decode_refresh_token(payload.refresh_token.clone()) {
        Ok(data) => data,
        Err(_) => {
            return common_response(StatusCode::UNAUTHORIZED, "Invalid or expired refresh token")
        }
    };

    let email = token_data.claims.email;

    let new_access_token = match encode_access_token(email.clone()) {
        Ok(token) => token,
        Err(_) => {
            return common_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate access token",
            )
        }
    };

    let new_refresh_token = match encode_refresh_token(email.clone()) {
        Ok(token) => token,
        Err(_) => {
            return common_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate refresh token",
            )
        }
    };

    let auth_response = ResponseSuccessDto {
        data: AuthTokenItemDto {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
        },
    };

    success_response(auth_response)
}
