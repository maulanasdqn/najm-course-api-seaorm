use super::{
	auth_dto::{
		AuthDataDto, AuthForgotRequestDto, AuthLoginRequestDto,
		AuthNewPasswordRequestDto, AuthRefreshTokenRequestDto,
		AuthRegisterRequestDto, AuthTokenItemDto, AuthVerifyEmailRequestDto,
	},
	AuthUsersItemDto,
};
use crate::{
	common_response, connect_redis, decode_access_token, decode_refresh_token,
	encode_access_token, encode_refresh_token, get_db, hash_password,
	permissions::PermissionsItemDto,
	roles::{RolesEnum, RolesItemDto},
	schemas::{
		PermissionsEntity, RolesColumn, RolesEntity, RolesPermissionsColumn,
		RolesPermissionsEntity, UsersActiveModel, UsersColumn, UsersEntity,
		UsersRelation,
	},
	send_email, success_response, verify_password, OtpManager, ResponseSuccessDto,
};
use axum::{http::StatusCode, response::Response, Json};
use chrono::Utc;
use email_address::EmailAddress;
use redis::Commands;
use sea_orm::{
	prelude::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
	JoinType, QueryFilter, QuerySelect, RelationTrait, Set,
};
use std::env;
use uuid::Uuid;

pub async fn mutation_login(
	Json(credentials): Json<AuthLoginRequestDto>,
) -> Response {
	if credentials.email.is_empty() {
		return common_response(StatusCode::BAD_REQUEST, "Email are required");
	}

	if credentials.password.is_empty() {
		return common_response(StatusCode::BAD_REQUEST, "Password are required");
	}

	if !EmailAddress::is_valid(&credentials.email) {
		return common_response(StatusCode::BAD_REQUEST, "Email not valid");
	}

	let db = get_db().await;

	let mut redis = connect_redis();

	let user_data = match UsersEntity::find()
		.select_only()
		.column(UsersColumn::Id)
		.column(UsersColumn::Email)
		.column(UsersColumn::Password)
		.column(UsersColumn::IsActive)
		.column(UsersColumn::IsProfileCompleted)
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
			is_profile_completed,
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
			is_profile_completed,
			fullname,
			avatar,
			phone_number,
			role_id,
			role_name,
		),
		Ok(None) => {
			return common_response(
				StatusCode::UNAUTHORIZED,
				"Email or password invalid",
			)
		}
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let (
		id,
		email,
		hashed_password,
		is_active,
		is_profile_completed,
		fullname,
		avatar,
		phone_number,
		role_id,
		role_name,
	) = user_data;

	if !verify_password(&credentials.password, &hashed_password).unwrap_or(false) {
		return common_response(
			StatusCode::UNAUTHORIZED,
			"Email or password invalid",
		);
	}

	if !is_active {
		return common_response(
			StatusCode::FORBIDDEN,
			"Your account is not active, please verify your email",
		);
	}

	let access_token = match encode_access_token(&email) {
		Ok(token) => token,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let refresh_token = match encode_refresh_token(&email) {
		Ok(token) => token,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

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
				is_profile_completed,
				phone_number,
				role: role_dto,
			},
		},
	};

	let redis_key = format!("authenticated_users_data:{}", credentials.email);

	match redis.set_ex::<_, String, ()>(
		&redis_key,
		serde_json::to_string(&response.data.user).unwrap_or_default(),
		86400,
	) {
		Ok(_) => success_response(response),
		Err(err) => common_response(
			StatusCode::INTERNAL_SERVER_ERROR,
			&format!("Redis storage failed: {}", err),
		),
	}
}

pub async fn mutation_register(new_user: Json<AuthRegisterRequestDto>) -> Response {
	if new_user.email.is_empty() {
		return common_response(StatusCode::BAD_REQUEST, "Email is required");
	}

	if new_user.password.is_empty() {
		return common_response(StatusCode::BAD_REQUEST, "Password is required");
	}

	if !EmailAddress::is_valid(&new_user.email) {
		return common_response(StatusCode::BAD_REQUEST, "Email is not valid");
	}

	if new_user.password.len() < 8 {
		return common_response(
			StatusCode::BAD_REQUEST,
			"Password must be at least 8 characters long",
		);
	}

	if new_user.fullname.is_empty() {
		return common_response(StatusCode::BAD_REQUEST, "Fullname is required");
	}

	if new_user.phone_number.is_empty() {
		return common_response(StatusCode::BAD_REQUEST, "Phone number is required");
	}

	if new_user.phone_number.len() < 10 {
		return common_response(
			StatusCode::BAD_REQUEST,
			"Phone number must be at least 10 characters",
		);
	}

	let db: DatabaseConnection = get_db().await;

	println!("Searching for user with email: {}", new_user.email);

	let check_email = UsersEntity::find()
		.select_only()
		.column(UsersColumn::Email)
		.filter(UsersColumn::Email.eq(new_user.email.clone()))
		.into_tuple::<String>()
		.one(&db)
		.await;

	match check_email {
		Ok(Some(_)) => {
			return common_response(
				StatusCode::CONFLICT,
				"User with that email already exists",
			);
		}
		Ok(None) => {
			println!("No user found with the given email.");
		}
		Err(err) => {
			println!("Query error: {:?}", err);
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				"An error occurred while checking user existence",
			);
		}
	}

	let hashed_password = match hash_password(&new_user.password) {
		Ok(password) => password,
		Err(_) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Error occurred while hashing the password",
			);
		}
	};

	let redis = connect_redis();
	let otp_manager = OtpManager::new(300);
	let otp = otp_manager.generate_otp(redis, &new_user.email);

	let student_role = match RolesEntity::find()
		.select_only()
		.column(RolesColumn::Id)
		.column(RolesColumn::Name)
		.filter(RolesColumn::Name.eq(RolesEnum::Student.to_string()))
		.one(&db)
		.await
	{
		Ok(Some(role)) => role,
		Ok(None) => {
			return common_response(
				StatusCode::BAD_REQUEST,
				"Student role not found",
			)
		}
		Err(err) => {
			println!("Role query error: {:?}", err);
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Error fetching roles",
			);
		}
	};

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

	if let Err(err) = send_email(&new_user.email, "Verification", email_content) {
		println!("Email sending error: {:?}", err);
		return common_response(
			StatusCode::INTERNAL_SERVER_ERROR,
			"Failed to send verification email",
		);
	}

	match active_model.insert(&db).await {
		Ok(_) => common_response(StatusCode::CREATED, "User created successfully"),
		Err(err) => {
			println!("Insert error: {:?}", err);
			common_response(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
		}
	}
}

pub async fn mutation_forgot_password(
	Json(payload): Json<AuthForgotRequestDto>,
) -> Response {
	let db: DatabaseConnection = get_db().await;

	println!("Received email: {}", payload.email);

	if payload.email.is_empty() {
		return common_response(StatusCode::BAD_REQUEST, "Email is required");
	}

	let email_lower = payload.email.to_lowercase();

	let user_result = UsersEntity::find()
		.select_only()
		.column(UsersColumn::Email)
		.filter(Expr::col(UsersColumn::Email).eq(email_lower.clone()))
		.into_tuple::<String>()
		.one(&db)
		.await;

	match user_result {
		Ok(Some(user_email)) => {
			println!("User found with email: {}", user_email);

			let mut redis = connect_redis();
			let reset_token = match encode_access_token(&user_email) {
				Ok(token) => token,
				Err(err) => {
					println!("Error generating reset token: {:?}", err);
					return common_response(
						StatusCode::INTERNAL_SERVER_ERROR,
						"Failed to generate reset token",
					);
				}
			};

			let redis_key = format!("reset_password:{}", user_email);

			if let Err(err) = redis.set_ex::<_, _, ()>(
				&redis_key,
				reset_token.clone(),
				(3600 * 24).try_into().unwrap_or(86400),
			) {
				println!("Redis error: {:?}", err);
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&err.to_string(),
				);
			}

			let fe_url = env::var("FE_URL").unwrap_or_else(|_| "".to_string());
			let email_content = format!(
                "You have requested a password reset. Please click the link below to continue: {}/auth/reset-password?token={}",
                fe_url, reset_token
            );

			if let Err(err) =
				send_email(&user_email, "Reset Password Request", &email_content)
			{
				println!("Email sending error: {:?}", err);
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to send email",
				);
			}

			return common_response(StatusCode::OK, "Password reset token sent");
		}
		Ok(None) => {
			println!("No user found with email: {}", email_lower);
			common_response(StatusCode::NOT_FOUND, "User not found")
		}
		Err(err) => {
			println!("Database query error: {:?}", err);
			common_response(StatusCode::INTERNAL_SERVER_ERROR, "Database error")
		}
	}
}

pub async fn mutation_send_otp(
	Json(payload): Json<AuthForgotRequestDto>,
) -> Response {
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

pub async fn mutation_verify_email(
	Json(payload): Json<AuthVerifyEmailRequestDto>,
) -> Response {
	let db: DatabaseConnection = get_db().await;
	let redis = connect_redis();
	let otp_manager = OtpManager::new(300);

	if payload.email.is_empty() {
		return common_response(StatusCode::BAD_REQUEST, "Email is required");
	}

	let is_valid = otp_manager.validate_otp(redis, &payload.email, payload.otp);

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
				return common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&err.to_string(),
				);
			}

			return common_response(StatusCode::OK, "Email successfully verified");
		}
	}

	common_response(StatusCode::BAD_REQUEST, "Invalid OTP")
}

pub async fn mutation_new_password(
	Json(payload): Json<AuthNewPasswordRequestDto>,
) -> Response {
	let db: DatabaseConnection = get_db().await;
	let mut redis = connect_redis();

	if payload.token.is_empty() || payload.password.is_empty() {
		return common_response(
			StatusCode::BAD_REQUEST,
			"Token and password are required",
		);
	}

	if payload.password.len() < 8 {
		return common_response(
			StatusCode::BAD_REQUEST,
			"Password must be at least 8 characters long",
		);
	}

	let tok = decode_access_token(&payload.token);

	let email = tok.unwrap().claims.email;
	let key = format!("reset_password:{}", email);

	let stored_token: Option<String> = redis.get(&key).ok();

	if stored_token.as_deref() != Some(&payload.token) {
		return common_response(
			StatusCode::BAD_REQUEST,
			"Invalid or expired reset token",
		);
	}

	let hashed_password = hash_password(&payload.password).unwrap();

	if let Some(user) = UsersEntity::find()
		.filter(UsersColumn::Email.eq(email.clone()))
		.one(&db)
		.await
		.ok()
		.flatten()
	{
		let mut active_user: UsersActiveModel = user.into();
		active_user.password = Set(hashed_password);

		if let Err(err) = active_user.update(&db).await {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			);
		}

		let _: () = redis.del(&key).unwrap_or(());

		return common_response(StatusCode::OK, "Password updated successfully");
	}

	common_response(StatusCode::NOT_FOUND, "User not found")
}

pub async fn mutation_refresh(
	Json(payload): Json<AuthRefreshTokenRequestDto>,
) -> Response {
	if payload.refresh_token.is_empty() {
		return common_response(
			StatusCode::BAD_REQUEST,
			"Refresh token is required",
		);
	}

	let token_data = match decode_refresh_token(&payload.refresh_token) {
		Ok(data) => data,
		Err(err) => {
			return common_response(StatusCode::UNAUTHORIZED, &err.to_string())
		}
	};

	let email = token_data.claims.email;

	let new_access_token = match encode_access_token(&email) {
		Ok(token) => token,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
			)
		}
	};

	let new_refresh_token = match encode_refresh_token(&email) {
		Ok(token) => token,
		Err(err) => {
			return common_response(
				StatusCode::INTERNAL_SERVER_ERROR,
				&err.to_string(),
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
