use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Utc;
use sea_orm::{entity::*, ActiveModelTrait, DatabaseConnection, PaginatorTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;

use crate::apps::roles::roles_dto::RolesItemDto;
use crate::libs::database::schemas::app_users_schema::ActiveModel as UserActiveModel;
use crate::utils::meta::{TMetaRequest, TMetaResponse};
use crate::{libs::database::get_db, utils::password::hash_password};

use crate::libs::database::schemas::{
    app_roles_schema::Column as RoleColumn, app_roles_schema::Entity as Role,
    app_users_schema::Column as UserColumn, app_users_schema::Entity as User,
};

use super::users_dto::{
    UsersDetailResponseDto, UsersItemDto, UsersListResponseDto, UsersRequestDto,
};

pub async fn mutation_create_users(new_user: Json<UsersRequestDto>) -> Response {
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

pub async fn query_get_user_by_id(id: Uuid) -> Json<UsersDetailResponseDto> {
    let db: DatabaseConnection = get_db().await;

    match User::find()
        .filter(UserColumn::Id.eq(id.clone()))
        .one(&db)
        .await
    {
        Ok(Some(user)) => {
            let role = Role::find()
                .filter(RoleColumn::Id.eq(user.role_id))
                .one(&db)
                .await
                .unwrap_or(None)
                .map(|r| RolesItemDto {
                    id: r.id.to_string(),
                    name: r.name,
                    permissions: vec![],
                    created_at: r.created_at.map(|dt| dt.to_string()),
                    updated_at: r.updated_at.map(|dt| dt.to_string()),
                });

            let user_detail = UsersItemDto {
                id: user.id.to_string(),
                fullname: user.fullname,
                email: user.email,
                avatar: user.avatar,
                phone_number: user.phone_number,
                password: user.password,
                referral_code: user.referral_code,
                referred_by: user.referred_by,
                role,
                created_at: user.created_at.map(|dt| dt.to_string()),
                updated_at: user.updated_at.map(|dt| dt.to_string()),
            };

            Json(UsersDetailResponseDto { data: user_detail })
        }
        Ok(None) => Json(UsersDetailResponseDto {
            data: UsersItemDto {
                id: "".to_string(),
                fullname: "".to_string(),
                email: "".to_string(),
                avatar: None,
                password: "".to_string(),
                phone_number: "".to_string(),
                referral_code: None,
                referred_by: None,
                role: None,
                created_at: None,
                updated_at: None,
            },
        }),
        Err(_) => Json(UsersDetailResponseDto {
            data: UsersItemDto {
                id: "".to_string(),
                fullname: "".to_string(),
                email: "".to_string(),
                avatar: None,
                phone_number: "".to_string(),
                password: "".to_string(),
                referral_code: None,
                referred_by: None,
                role: None,
                created_at: None,
                updated_at: None,
            },
        }),
    }
}

pub async fn query_get_user_by_email(email: String) -> Json<UsersDetailResponseDto> {
    let db: DatabaseConnection = get_db().await;

    match User::find()
        .filter(UserColumn::Email.eq(email.clone()))
        .one(&db)
        .await
    {
        Ok(Some(user)) => {
            let role = Role::find()
                .filter(RoleColumn::Id.eq(user.role_id))
                .one(&db)
                .await
                .unwrap_or(None)
                .map(|r| RolesItemDto {
                    id: r.id.to_string(),
                    name: r.name,
                    permissions: vec![],
                    created_at: r.created_at.map(|dt| dt.to_string()),
                    updated_at: r.updated_at.map(|dt| dt.to_string()),
                });

            let user_detail = UsersItemDto {
                id: user.id.to_string(),
                fullname: user.fullname,
                email: user.email,
                avatar: user.avatar,
                phone_number: user.phone_number,
                password: user.password,
                referral_code: user.referral_code,
                referred_by: user.referred_by,
                role,
                created_at: user.created_at.map(|dt| dt.to_string()),
                updated_at: user.updated_at.map(|dt| dt.to_string()),
            };

            Json(UsersDetailResponseDto { data: user_detail })
        }
        Ok(None) => Json(UsersDetailResponseDto {
            data: UsersItemDto {
                id: "".to_string(),
                fullname: "".to_string(),
                email: "".to_string(),
                avatar: None,
                password: "".to_string(),
                phone_number: "".to_string(),
                referral_code: None,
                referred_by: None,
                role: None,
                created_at: None,
                updated_at: None,
            },
        }),
        Err(_) => Json(UsersDetailResponseDto {
            data: UsersItemDto {
                id: "".to_string(),
                fullname: "".to_string(),
                email: "".to_string(),
                avatar: None,
                phone_number: "".to_string(),
                password: "".to_string(),
                referral_code: None,
                referred_by: None,
                role: None,
                created_at: None,
                updated_at: None,
            },
        }),
    }
}

pub async fn query_get_users(params: TMetaRequest) -> Json<UsersListResponseDto> {
    let db: DatabaseConnection = get_db().await;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    let paginator = User::find().paginate(&db, per_page.into());
    let total_items = paginator.num_items().await.unwrap_or(0);

    match paginator.fetch_page(page - 1).await {
        Ok(users) => {
            let mut data = Vec::new();

            for user in users {
                let roles = Role::find()
                    .filter(RoleColumn::Id.eq(user.role_id))
                    .all(&db)
                    .await
                    .unwrap_or_else(|_| vec![]);

                let role = roles.into_iter().next().map(|r| RolesItemDto {
                    id: r.id.to_string(),
                    name: r.name,
                    permissions: vec![],
                    created_at: r.created_at.map(|dt| dt.to_string()),
                    updated_at: r.updated_at.map(|dt| dt.to_string()),
                });

                data.push(UsersItemDto {
                    id: user.id.to_string(),
                    password: "".to_string(),
                    fullname: user.fullname,
                    email: user.email,
                    avatar: user.avatar,
                    phone_number: user.phone_number,
                    referral_code: user.referral_code,
                    referred_by: user.referred_by,
                    created_at: user.created_at.map(|dt| dt.to_string()),
                    updated_at: user.updated_at.map(|dt| dt.to_string()),
                    role,
                });
            }

            Json(UsersListResponseDto {
                data,
                meta: Some(TMetaResponse {
                    page: Some(page),
                    per_page: Some(per_page),
                    total: Some(total_items),
                }),
            })
        }
        Err(_) => Json(UsersListResponseDto {
            data: vec![],
            meta: Some(TMetaResponse {
                page: Some(page),
                per_page: Some(per_page),
                total: Some(0),
            }),
        }),
    }
}
