use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Utc;
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
use crate::apps::v1::roles::roles_repository::query_get_role_by_id;
use crate::get_version;
use crate::libs::database::schemas::app_users_schema::ActiveModel as UserActiveModel;
use crate::libs::database::schemas::{
    app_roles_schema::Entity as Role, app_users_schema::Column as UserColumn,
    app_users_schema::Entity as User, app_users_schema::Relation as UserRelation,
};
use crate::utils::dto::{MetaRequestDto, MetaResponseDto};
use crate::utils::error::AppError;
use crate::{libs::database::get_db, utils::password::hash_password};

pub async fn mutation_create_users(
    new_user: Json<UsersCreateRequestDto>,
) -> Result<Response, AppError> {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    new_user
        .validate()
        .map_err(|_| AppError::ValidationFailed)?;

    if let Some(_) = User::find()
        .filter(UserColumn::Email.eq(new_user.email.clone()))
        .one(&db)
        .await?
    {
        return Err(AppError::ValidationFailed);
    }

    let hashed_password = hash_password(&new_user.password).map_err(|_| AppError::InternalError)?;

    let active_model = UserActiveModel {
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
        otp: Set(None),
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

pub async fn query_get_user_by_id(id: Uuid) -> Result<Json<UsersDetailResponseDto>, AppError> {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    if let Some(user) = User::find().filter(UserColumn::Id.eq(id)).one(&db).await? {
        let role = Some(
            query_get_role_by_id(user.role_id)
                .await
                .unwrap()
                .data
                .clone(),
        );

        let user_detail = UsersItemDto {
            id: user.id.to_string(),
            fullname: user.fullname,
            email: user.email,
            avatar: user.avatar,
            phone_number: user.phone_number,
            referral_code: user.referral_code,
            referred_by: user.referred_by,
            role,
            created_at: user.created_at.map(|dt| dt.to_string()),
            updated_at: user.updated_at.map(|dt| dt.to_string()),
        };

        Ok(Json(UsersDetailResponseDto {
            data: user_detail,
            version,
        }))
    } else {
        Err(AppError::NotFound)
    }
}

pub async fn query_get_users(params: MetaRequestDto) -> Json<UsersListResponseDto> {
    let db: DatabaseConnection = get_db().await;
    let version = get_version().unwrap();

    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(10).max(1).min(100);

    let paginator = User::find()
        .filter(UserColumn::IsDeleted.eq(false))
        .join(JoinType::LeftJoin, UserRelation::Role.def())
        .select_also(Role)
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
