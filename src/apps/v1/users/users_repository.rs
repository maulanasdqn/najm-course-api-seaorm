use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{DateTime, Utc};
use sea_orm::{
    entity::*, ActiveModelTrait, DatabaseConnection, JoinType, PaginatorTrait, QueryFilter,
    QuerySelect, Set,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use super::users_dto::{
    UsersCreateRequestDto, UsersDetailResponseDto, UsersItemDto, UsersItemListDto,
    UsersListResponseDto,
};
use super::UsersCheckLoginDto;
use crate::roles::query_get_role_by_id;
use crate::schemas::{RolesEntity, UsersActiveModel, UsersColumn, UsersEntity, UsersRelation};
use crate::{get_db, get_version, hash_password, AppError, MetaRequestDto, MetaResponseDto};

pub async fn mutation_create_users(
    new_user: Json<UsersCreateRequestDto>,
) -> Result<Response, AppError> {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    new_user
        .validate()
        .map_err(|_| AppError::ValidationFailed)?;

    if let Some(_) = UsersEntity::find()
        .filter(UsersColumn::Email.eq(new_user.email.clone()))
        .one(&db)
        .await?
    {
        return Err(AppError::ValidationFailed);
    }

    let hashed_password = hash_password(&new_user.password).map_err(|_| AppError::InternalError)?;

    let active_model = UsersActiveModel {
        id: Set(Uuid::new_v4()),
        role_id: Set(Uuid::parse_str(new_user.role_id.as_str()).unwrap_or(Uuid::new_v4())),
        fullname: Set(new_user.fullname.clone()),
        email: Set(new_user.email.clone()),
        email_verified: Set(Some(Utc::now())),
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

    active_model
        .insert(&db)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok((
        StatusCode::CREATED,
        Json(json!({ "message": "User created successfully", "version": version })),
    )
        .into_response())
}

pub async fn query_get_user_by_id(
    id_payload: String,
) -> Result<Json<UsersDetailResponseDto>, AppError> {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    if let Some((
        id,
        email,
        fullname,
        avatar,
        phone_number,
        referral_code,
        referred_by,
        role_id,
        created_at,
        updated_at,
    )) = UsersEntity::find()
        .select_only()
        .column(UsersColumn::Id)
        .column(UsersColumn::Email)
        .column(UsersColumn::Fullname)
        .column(UsersColumn::Avatar)
        .column(UsersColumn::PhoneNumber)
        .column(UsersColumn::ReferralCode)
        .column(UsersColumn::ReferredBy)
        .column(UsersColumn::RoleId)
        .column(UsersColumn::CreatedAt)
        .column(UsersColumn::UpdatedAt)
        .filter(UsersColumn::Id.eq(id_payload))
        .into_tuple::<(
            String,
            String,
            String,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            String,
            Option<DateTime<Utc>>,
            Option<DateTime<Utc>>,
        )>()
        .one(&db)
        .await?
    {
        let role = Some(
            query_get_role_by_id(Uuid::parse_str(&role_id).unwrap())
                .await
                .unwrap()
                .data
                .clone(),
        );

        let user_detail = UsersItemDto {
            id,
            email,
            fullname,
            avatar,
            phone_number,
            referral_code,
            referred_by,
            role,
            created_at: created_at.map(|dt| dt.to_string()),
            updated_at: updated_at.map(|dt| dt.to_string()),
        };

        Ok(Json(UsersDetailResponseDto {
            data: user_detail,
            version,
        }))
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn query_get_user_by_email(
    email_payload: String,
) -> Result<Json<UsersDetailResponseDto>, AppError> {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    if let Some((
        id,
        email,
        fullname,
        avatar,
        phone_number,
        referral_code,
        referred_by,
        role_id,
        created_at,
        updated_at,
    )) = UsersEntity::find()
        .select_only()
        .column(UsersColumn::Id)
        .column(UsersColumn::Email)
        .column(UsersColumn::Fullname)
        .column(UsersColumn::Avatar)
        .column(UsersColumn::PhoneNumber)
        .column(UsersColumn::ReferralCode)
        .column(UsersColumn::ReferredBy)
        .column(UsersColumn::RoleId)
        .column(UsersColumn::CreatedAt)
        .column(UsersColumn::UpdatedAt)
        .filter(UsersColumn::Email.eq(email_payload))
        .into_tuple::<(
            String,
            String,
            String,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            String,
            Option<DateTime<Utc>>,
            Option<DateTime<Utc>>,
        )>()
        .one(&db)
        .await?
    {
        let role = Some(
            query_get_role_by_id(Uuid::parse_str(&role_id).unwrap())
                .await
                .unwrap()
                .data
                .clone(),
        );

        let user_detail = UsersItemDto {
            id,
            email,
            fullname,
            avatar,
            phone_number,
            referral_code,
            referred_by,
            role,
            created_at: created_at.map(|dt| dt.to_string()),
            updated_at: updated_at.map(|dt| dt.to_string()),
        };

        Ok(Json(UsersDetailResponseDto {
            data: user_detail,
            version,
        }))
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn query_password_and_is_active(email: String) -> Result<UsersCheckLoginDto, AppError> {
    let db: DatabaseConnection = get_db().await;

    if let Some(user) = UsersEntity::find()
        .select_only()
        .column(UsersColumn::Password)
        .column(UsersColumn::IsActive)
        .filter(UsersColumn::Email.eq(email))
        .one(&db)
        .await?
    {
        Ok(UsersCheckLoginDto {
            password: user.password,
            is_active: user.is_active,
        })
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn query_get_users(params: MetaRequestDto) -> Json<UsersListResponseDto> {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(10).max(1).min(100);

    let paginator = UsersEntity::find()
        .filter(UsersColumn::IsDeleted.eq(false))
        .join(JoinType::LeftJoin, UsersRelation::Role.def())
        .select_also(RolesEntity)
        .paginate(&db, per_page);

    let total_items = paginator.num_items().await.unwrap_or(0);

    let results = paginator.fetch_page(page - 1).await.unwrap_or(vec![]);
    let data: Vec<UsersItemListDto> = results
        .into_iter()
        .map(|(user, role)| UsersItemListDto {
            id: user.id.to_string(),
            fullname: user.fullname,
            email: user.email,
            avatar: user.avatar,
            phone_number: user.phone_number,
            referral_code: user.referral_code,
            referred_by: user.referred_by,
            role: role.map(|r| r.name).unwrap_or("-".to_string()),
            created_at: user.created_at.map(|dt| dt.to_string()),
            updated_at: user.updated_at.map(|dt| dt.to_string()),
        })
        .collect();

    Json(UsersListResponseDto {
        data,
        meta: Some(MetaResponseDto {
            page: Some(page),
            per_page: Some(per_page),
            total: Some(total_items),
        }),
        version,
    })
}
