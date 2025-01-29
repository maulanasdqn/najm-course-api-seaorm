use super::{
    auth_dto::{
        AuthDataDto, AuthForgotRequestDto, AuthLoginRequestDto, AuthNewPasswordRequestDto,
        AuthRefreshTokenRequestDto, AuthRegisterRequestDto, AuthTokenItemDto,
        AuthVerifyEmailRequestDto,
    },
    AuthUsersItemDto,
};
use crate::{
    common_response, connect_redis, decode_access_token, decode_refresh_token, encode_access_token,
    encode_refresh_token, get_db, hash_password,
    permissions::PermissionsItemDto,
    roles::{RolesEnum, RolesItemDto},
    schemas::{
        PermissionsEntity, RolesColumn, RolesEntity, RolesPermissionsColumn,
        RolesPermissionsEntity, UsersActiveModel, UsersColumn, UsersEntity, UsersRelation,
    },
    send_email, success_response, verify_password, OtpManager, ResponseSuccessDto,
};
use axum::{http::StatusCode, response::Response, Json};
use chrono::Utc;
use redis::Commands;
use sea_orm::{
    prelude::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, JoinType,
    QueryFilter, QuerySelect, RelationTrait, Set,
};
use std::env;
use uuid::Uuid;

pub async fn mutation_login(Json(credentials): Json<AuthLoginRequestDto>) -> Response {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return common_response(StatusCode::BAD_REQUEST, "Email and password are required");
    }

    let db = get_db().await;

    let user_data = match UsersEntity::find()
        .select_only()
        .column(UsersColumn::Id)
        .column(UsersColumn::Email)
        .column(UsersColumn::Password)
        .column(UsersColumn::IsActive)
        .column(UsersColumn::Fullname)
        .column(UsersColumn::Avatar)
        .column(UsersColumn::PhoneNumber)
        .column_as(Expr::col((RolesEntity, RolesColumn::Id)), "role_id")
        .column_as(Expr::col((RolesEntity, RolesColumn::Name)), "role_name")
        .filter(UsersColumn::Email.eq(credentials.email.clone()))
        .join(JoinType::LeftJoin, UsersRelation::Role.def())
        .into_tuple::<(
            Uuid,
            String,
            String,
            bool,
            String,
            Option<String>,
            String,
            Option<Uuid>,
            Option<String>,
        )>()
        .one(&db)
        .await
    {
        Ok(Some((
            id,
            email,
            hashed_password,
            is_active,
            fullname,
            avatar,
            phone_number,
            role_id,
            role_name,
        ))) => (
            id,
            email,
            hashed_password,
            is_active,
            fullname,
            avatar,
            phone_number,
            role_id,
            role_name,
        ),
        Ok(None) => return common_response(StatusCode::UNAUTHORIZED, "Email or password invalid"),
        Err(err) => return common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    };

    let (id, email, hashed_password, is_active, fullname, avatar, phone_number, role_id, role_name) =
        user_data;

    if !verify_password(&credentials.password, &hashed_password).unwrap_or(false) {
        return common_response(StatusCode::UNAUTHORIZED, "Email or password invalid");
    }

    if !is_active {
        return common_response(
            StatusCode::FORBIDDEN,
            "Your account is not active, please verify your email",
        );
    }

    let access_token = encode_access_token(email.clone()).unwrap();
    let refresh_token = encode_refresh_token(email.clone()).unwrap();

    let permissions = if let Some(role_id) = role_id {
        match RolesPermissionsEntity::find()
            .filter(RolesPermissionsColumn::RoleId.eq(role_id))
            .find_also_related(PermissionsEntity)
            .all(&db)
            .await
        {
            Ok(data) => data
                .into_iter()
                .filter_map(|(_, permission)| {
                    permission.map(|perm| PermissionsItemDto {
                        id: perm.id.to_string(),
                        name: perm.name,
                        created_at: perm.created_at.map(|dt| dt.to_string()),
                        updated_at: perm.updated_at.map(|dt| dt.to_string()),
                    })
                })
                .collect::<Vec<PermissionsItemDto>>(),
            Err(_) => vec![],
        }
    } else {
        vec![]
    };

    let role_dto = match (role_id, role_name) {
        (Some(id), Some(name)) => Some(RolesItemDto {
            id: id.to_string(),
            name,
            permissions,
            created_at: None,
            updated_at: None,
        }),
        _ => None,
    };

    let response = ResponseSuccessDto {
        data: AuthDataDto {
            token: AuthTokenItemDto {
                access_token,
                refresh_token,
            },
            user: AuthUsersItemDto {
                id: id.to_string(),
                email,
                fullname,
                avatar,
                phone_number,
                role: role_dto,
            },
        },
    };

    success_response(response)
}

pub async fn mutation_register(new_user: Json<AuthRegisterRequestDto>) -> Response {
    let redis = connect_redis();

    let db: DatabaseConnection = get_db().await;

    let otp_manager = OtpManager::new(300);

    if let Ok(Some(_)) = UsersEntity::find()
        .select_only()
        .column(UsersColumn::Email)
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

    let student_role = RolesEntity::find()
        .select_only()
        .column(RolesColumn::Id)
        .column(RolesColumn::Name)
        .filter(RolesColumn::Name.eq(RolesEnum::Student.to_string()))
        .one(&db)
        .await
        .unwrap();

    let active_model = UsersActiveModel {
        id: Set(Uuid::new_v4()),
        role_id: Set(student_role.unwrap().id),
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
        .select_only()
        .column(UsersColumn::Email)
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
        data: Some(AuthTokenItemDto {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
        }),
    };

    success_response(auth_response)
}
