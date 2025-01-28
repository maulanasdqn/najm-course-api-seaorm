use axum::http::StatusCode;
use axum::response::Response;
use axum::Json;
use chrono::{NaiveDate, Utc};
use hyper::HeaderMap;
use prelude::Expr;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    entity::*, ActiveModelTrait, DatabaseConnection, JoinType, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use uuid::Uuid;

use super::users_dto::{UsersCreateRequestDto, UsersItemDto, UsersItemListDto};
use super::{UsersActiveInactiveRequestDto, UsersUpdateRequestDto};
use crate::permissions::PermissionsItemDto;
use crate::roles::RolesItemDto;
use crate::schemas::{
    PermissionsColumn, PermissionsEntity, RolesEntity, RolesPermissionsColumn,
    RolesPermissionsEntity, UsersActiveModel, UsersColumn, UsersEntity, UsersRelation,
};
use crate::{
    common_response, decode_access_token, get_db, hash_password, success_response,
    success_response_list, MetaRequestDto, MetaResponseDto, ResponseSuccessDto,
    ResponseSuccessListDto,
};

pub async fn mutation_create_users(new_user: Json<UsersCreateRequestDto>) -> Response {
    let db: DatabaseConnection = get_db().await;

    if new_user.password.len() < 8 {
        return common_response(
            StatusCode::BAD_REQUEST,
            "Password must be have 8 character long",
        );
    }

    if let Ok(Some(_)) = UsersEntity::find()
        .filter(UsersColumn::Email.eq(new_user.email.clone()))
        .one(&db)
        .await
    {
        return common_response(StatusCode::CONFLICT, "User with that email already exist");
    }

    let hashed_password = hash_password(&new_user.password).unwrap();

    let active_model = UsersActiveModel {
        id: Set(Uuid::new_v4()),
        role_id: Set(Uuid::parse_str(&new_user.role_id).unwrap()),
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

    match active_model.insert(&db).await {
        Ok(_) => common_response(StatusCode::CREATED, "User created successfully"),
        Err(err) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    }
}

pub async fn query_get_user_me(headers: HeaderMap) -> Response {
    let db = get_db().await;

    let auth_header = match headers.get("Authorization") {
        Some(header) => header.to_str(),
        None => return common_response(StatusCode::FORBIDDEN, "You are not authorized"),
    };

    let auth_header = match auth_header {
        Ok(header) => header,
        Err(_) => return common_response(StatusCode::BAD_REQUEST, "Invalid header format"),
    };

    let mut header_parts = auth_header.split_whitespace();

    let token = match header_parts.nth(1) {
        Some(token) => token,
        None => return common_response(StatusCode::BAD_REQUEST, "Invalid token format"),
    };

    let token_data = match decode_access_token(token.to_string()) {
        Ok(data) => data,
        Err(_) => return common_response(StatusCode::UNAUTHORIZED, "Invalid or expired token"),
    };

    let email = token_data.claims.email;

    match UsersEntity::find()
        .filter(UsersColumn::Email.eq(email))
        .find_also_related(RolesEntity)
        .one(&db)
        .await
    {
        Ok(Some((user, Some(role)))) => {
            let permissions = PermissionsEntity::find()
                .join(
                    JoinType::InnerJoin,
                    RolesPermissionsEntity::belongs_to(PermissionsEntity)
                        .from(RolesPermissionsColumn::PermissionId)
                        .to(PermissionsColumn::Id)
                        .into(),
                )
                .filter(RolesPermissionsColumn::RoleId.eq(role.id))
                .all(&db)
                .await
                .unwrap_or_default();

            let role_permissions = permissions
                .into_iter()
                .map(|perm| PermissionsItemDto {
                    id: perm.id.to_string(),
                    name: perm.name,
                    created_at: perm.created_at.map(|dt| dt.to_string()),
                    updated_at: perm.updated_at.map(|dt| dt.to_string()),
                })
                .collect::<Vec<PermissionsItemDto>>();

            let role_dto = RolesItemDto {
                id: role.id.to_string(),
                name: role.name,
                permissions: role_permissions,
                created_at: role.created_at.map(|dt| dt.to_string()),
                updated_at: role.updated_at.map(|dt| dt.to_string()),
            };

            let user_detail = UsersItemDto {
                id: user.id.to_string(),
                email: user.email,
                fullname: user.fullname,
                avatar: user.avatar,
                phone_number: user.phone_number,
                referral_code: user.referral_code,
                referred_by: user.referred_by,
                role: Some(role_dto),
                identity_number: user.identity_number,
                is_active: user.is_active,
                student_type: user.student_type,
                religion: user.religion,
                gender: user.gender,
                created_at: user.created_at.map(|dt| dt.to_string()),
                updated_at: user.updated_at.map(|dt| dt.to_string()),
            };

            let response = ResponseSuccessDto { data: user_detail };
            success_response(response)
        }
        Ok(Some((user, None))) => {
            let user_detail = UsersItemDto {
                id: user.id.to_string(),
                email: user.email,
                fullname: user.fullname,
                avatar: user.avatar,
                phone_number: user.phone_number,
                referral_code: user.referral_code,
                referred_by: user.referred_by,
                role: None,
                identity_number: user.identity_number,
                is_active: user.is_active,
                student_type: user.student_type,
                religion: user.religion,
                gender: user.gender,
                created_at: user.created_at.map(|dt| dt.to_string()),
                updated_at: user.updated_at.map(|dt| dt.to_string()),
            };
            let response = ResponseSuccessDto { data: user_detail };
            success_response(response)
        }
        Ok(None) => common_response(StatusCode::NOT_FOUND, "User not found"),
        Err(err) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    }
}

pub async fn query_get_user_by_id(id_payload: String) -> Response {
    let db = get_db().await;

    match UsersEntity::find()
        .filter(UsersColumn::Id.eq(Uuid::parse_str(&id_payload).unwrap()))
        .find_also_related(RolesEntity)
        .one(&db)
        .await
    {
        Ok(Some((user, Some(role)))) => {
            let permissions = PermissionsEntity::find()
                .join(
                    sea_orm::JoinType::InnerJoin,
                    RolesPermissionsEntity::belongs_to(PermissionsEntity)
                        .from(RolesPermissionsColumn::PermissionId)
                        .to(PermissionsColumn::Id)
                        .into(),
                )
                .filter(RolesPermissionsColumn::RoleId.eq(role.id))
                .all(&db)
                .await
                .unwrap_or_default();

            let role_permissions = permissions
                .into_iter()
                .map(|perm| PermissionsItemDto {
                    id: perm.id.to_string(),
                    name: perm.name,
                    created_at: perm.created_at.map(|dt| dt.to_string()),
                    updated_at: perm.updated_at.map(|dt| dt.to_string()),
                })
                .collect::<Vec<PermissionsItemDto>>();

            let role_dto = RolesItemDto {
                id: role.id.to_string(),
                name: role.name,
                permissions: role_permissions,
                created_at: role.created_at.map(|dt| dt.to_string()),
                updated_at: role.updated_at.map(|dt| dt.to_string()),
            };

            let user_detail = UsersItemDto {
                id: user.id.to_string(),
                email: user.email,
                fullname: user.fullname,
                avatar: user.avatar,
                phone_number: user.phone_number,
                referral_code: user.referral_code,
                referred_by: user.referred_by,
                role: Some(role_dto),
                identity_number: user.identity_number,
                is_active: user.is_active,
                student_type: user.student_type,
                religion: user.religion,
                gender: user.gender,
                created_at: user.created_at.map(|dt| dt.to_string()),
                updated_at: user.updated_at.map(|dt| dt.to_string()),
            };

            let response = ResponseSuccessDto { data: user_detail };
            success_response(response)
        }
        Ok(Some((user, None))) => {
            let user_detail = UsersItemDto {
                id: user.id.to_string(),
                email: user.email,
                fullname: user.fullname,
                avatar: user.avatar,
                phone_number: user.phone_number,
                referral_code: user.referral_code,
                referred_by: user.referred_by,
                role: None,
                identity_number: user.identity_number,
                is_active: user.is_active,
                student_type: user.student_type,
                religion: user.religion,
                gender: user.gender,
                created_at: user.created_at.map(|dt| dt.to_string()),
                updated_at: user.updated_at.map(|dt| dt.to_string()),
            };
            let response = ResponseSuccessDto { data: user_detail };
            success_response(response)
        }
        Ok(None) => common_response(StatusCode::NOT_FOUND, "User not found"),
        Err(err) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    }
}

pub async fn query_get_users(params: MetaRequestDto) -> Response {
    let db: DatabaseConnection = get_db().await;

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    let search = params.search.unwrap_or_default();
    let sort_by = params
        .sort_by
        .unwrap_or("created_at".to_string())
        .to_lowercase();
    let order = params.order.unwrap_or("desc".to_string()).to_lowercase();
    let filter = params.filter.unwrap_or_default();
    let filter_by = params.filter_by.unwrap_or_default().to_lowercase();

    let mut query = UsersEntity::find().filter(UsersColumn::IsDeleted.eq(false));

    if !search.is_empty() {
        query = query.filter(Expr::col(UsersColumn::Fullname).ilike(format!("%{}%", search)));
    }

    if filter_by == "role_id" && !filter.is_empty() {
        if let Ok(role_id) = Uuid::parse_str(&filter) {
            query = query.filter(UsersColumn::RoleId.eq(role_id));
        } else {
            return common_response(StatusCode::BAD_REQUEST, "Invalid role_id format");
        }
    }

    query = match (sort_by.as_str(), order.as_str()) {
        ("fullname", "asc") => query.order_by_asc(UsersColumn::Fullname),
        ("fullname", "desc") => query.order_by_desc(UsersColumn::Fullname),
        ("email", "asc") => query.order_by_asc(UsersColumn::Email),
        ("email", "desc") => query.order_by_desc(UsersColumn::Email),
        ("created_at", "asc") => query.order_by_asc(UsersColumn::CreatedAt),
        ("created_at", "desc") => query.order_by_desc(UsersColumn::CreatedAt),
        ("updated_at", "asc") => query.order_by_asc(UsersColumn::UpdatedAt),
        ("updated_at", "desc") => query.order_by_desc(UsersColumn::UpdatedAt),
        _ => query.order_by_asc(UsersColumn::CreatedAt),
    };

    let paginator = query
        .join(JoinType::LeftJoin, UsersRelation::Role.def())
        .select_also(RolesEntity)
        .paginate(&db, per_page);

    let total_items = match paginator.num_items().await {
        Ok(count) => count,
        Err(err) => {
            return common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string());
        }
    };

    let results = match paginator.fetch_page(page - 1).await {
        Ok(data) => data,
        Err(err) => {
            return common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string());
        }
    };

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
            role: role.map(|r| r.name).unwrap_or_else(|| "-".to_string()),
            created_at: user.created_at.map(|dt| dt.to_string()),
            updated_at: user.updated_at.map(|dt| dt.to_string()),
        })
        .collect();

    let users_response = ResponseSuccessListDto {
        data,
        meta: Some(MetaResponseDto {
            page: Some(page),
            per_page: Some(per_page),
            total: Some(total_items),
        }),
    };

    success_response_list(users_response)
}

pub async fn mutation_delete_user(user_id: String) -> Response {
    let db: DatabaseConnection = get_db().await;

    let user_id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return common_response(StatusCode::BAD_REQUEST, "Invalid user ID format"),
    };

    let user = match UsersEntity::find()
        .filter(UsersColumn::Id.eq(user_id))
        .filter(UsersColumn::IsDeleted.eq(false))
        .one(&db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return common_response(StatusCode::NOT_FOUND, "User not found"),
        Err(err) => return common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    };

    let mut active_model: UsersActiveModel = user.into();
    active_model.is_deleted = Set(true);
    active_model.updated_at = Set(Some(Utc::now()));

    match active_model.update(&db).await {
        Ok(_) => common_response(StatusCode::OK, "User deleted successfully"),
        Err(err) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    }
}

pub async fn mutation_update_user(
    id: String,
    Json(update_data): Json<UsersUpdateRequestDto>,
) -> Response {
    let db = get_db().await;

    let user_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return common_response(StatusCode::BAD_REQUEST, "Invalid user ID format"),
    };

    let user = match UsersEntity::find()
        .filter(UsersColumn::Id.eq(user_id))
        .filter(UsersColumn::IsDeleted.eq(false))
        .one(&db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return common_response(StatusCode::NOT_FOUND, "User not found"),
        Err(err) => return common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    };

    let mut active_model: UsersActiveModel = user.into();

    if let Some(email) = &update_data.email {
        active_model.email = Set(email.clone());
    }

    if let Some(fullname) = &update_data.fullname {
        active_model.fullname = Set(fullname.clone());
    }

    if let Some(avatar) = &update_data.avatar {
        active_model.avatar = Set(Some(avatar.clone()));
    }

    if let Some(phone_number) = &update_data.phone_number {
        active_model.phone_number = Set(phone_number.clone());
    }

    if let Some(role_id) = &update_data.role_id {
        active_model.role_id = Set(Uuid::parse_str(role_id).unwrap());
    }

    if let Some(student_type) = &update_data.student_type {
        active_model.student_type = Set(student_type.clone());
    }

    if let Some(birthdate) = &update_data.birthdate {
        match NaiveDate::parse_from_str(birthdate, "%Y-%m-%d") {
            Ok(parsed_date) => match parsed_date.and_hms_opt(0, 0, 0) {
                Some(naive_datetime) => {
                    let datetime = naive_datetime.and_local_timezone(Utc).unwrap();
                    active_model.birth_date = Set(Some(datetime));
                }
                None => {
                    return common_response(
                        StatusCode::BAD_REQUEST,
                        "Invalid time components in birthdate",
                    );
                }
            },
            Err(_) => {
                return common_response(
                    StatusCode::BAD_REQUEST,
                    "Invalid birthdate format. Use YYYY-MM-DD.",
                );
            }
        }
    }

    if let Some(gender) = &update_data.gender {
        active_model.gender = Set(Some(gender.clone()));
    }

    if let Some(identity_number) = &update_data.identity_number {
        active_model.identity_number = Set(Some(identity_number.clone()));
    }

    if let Some(religion) = &update_data.religion {
        active_model.religion = Set(Some(religion.clone()));
    }

    active_model.updated_at = Set(Some(Utc::now()));

    match active_model.update(&db).await {
        Ok(_) => common_response(StatusCode::OK, "User updated successfully"),
        Err(err) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    }
}

pub async fn mutation_set_active_inactive_user(
    id: String,
    Json(update_data): Json<UsersActiveInactiveRequestDto>,
) -> Response {
    let db = get_db().await;

    let user_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return common_response(StatusCode::BAD_REQUEST, "Invalid user ID format"),
    };

    let user = match UsersEntity::find()
        .filter(UsersColumn::Id.eq(user_id))
        .filter(UsersColumn::IsDeleted.eq(false))
        .one(&db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return common_response(StatusCode::NOT_FOUND, "User not found"),
        Err(err) => return common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    };

    let mut active_model: UsersActiveModel = user.into();
    active_model.is_active = Set(update_data.is_active);
    active_model.updated_at = Set(Some(Utc::now()));

    match active_model.update(&db).await {
        Ok(_) => common_response(StatusCode::OK, "User updated successfully"),
        Err(err) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string()),
    }
}
