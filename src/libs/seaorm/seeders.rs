use chrono::Utc;
use rand::seq::SliceRandom;
use sea_orm::{
	ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use super::schemas::{
	app_options_schema as options, app_permissions_schema as permissions,
	app_questions_schema as questions,
	app_roles_permissions_schema as roles_permissions, app_roles_schema as roles,
	app_sessions_has_tests_schema as sessions_has_tests,
	app_test_sessions_schema as test_sessions, app_tests_schema as tests,
	app_users_schema as users,
};
use crate::{hash_password, PermissionsEnum, RolesEnum};

pub async fn run_seeds(db: &DatabaseConnection) -> Result<(), DbErr> {
	println!("Seeding Permissions....");
	let mut permission_ids = Vec::new();
	for perm in [
		PermissionsEnum::ReadListUsers,
		PermissionsEnum::ReadDetailUsers,
		PermissionsEnum::CreateUsers,
		PermissionsEnum::DeleteUsers,
		PermissionsEnum::UpdateUsers,
		PermissionsEnum::ReadListRoles,
		PermissionsEnum::ReadDetailRoles,
		PermissionsEnum::CreateRoles,
		PermissionsEnum::DeleteRoles,
		PermissionsEnum::UpdateRoles,
		PermissionsEnum::ReadListPermissions,
		PermissionsEnum::ReadDetailPermissions,
		PermissionsEnum::CreatePermissions,
		PermissionsEnum::DeletePermissions,
		PermissionsEnum::UpdatePermissions,
		PermissionsEnum::ReadListSessions,
		PermissionsEnum::ReadDetailSessions,
		PermissionsEnum::CreateSessions,
		PermissionsEnum::UpdateSessions,
		PermissionsEnum::DeleteSessions,
		PermissionsEnum::ReadListTests,
		PermissionsEnum::ReadDetailTests,
		PermissionsEnum::CreateTests,
		PermissionsEnum::UpdateTests,
		PermissionsEnum::DeleteTests,
	]
	.iter()
	{
		let perm_str = perm.to_string();
		if let Some(existing) = permissions::Entity::find()
			.filter(permissions::Column::Name.eq(perm_str.clone()))
			.one(db)
			.await?
		{
			println!("Permission '{}' already exists. Skipping.", perm_str);
			permission_ids.push((perm.clone(), existing.id));
			continue;
		}
		let permission_id = Uuid::new_v4();
		let permission = permissions::ActiveModel {
			id: Set(permission_id),
			name: Set(perm_str.clone()),
			created_at: Set(Some(Utc::now())),
			updated_at: Set(Some(Utc::now())),
		};
		permissions::Entity::insert(permission).exec(db).await?;
		println!("Inserted permission '{}'.", perm_str);
		permission_ids.push((perm.clone(), permission_id));
	}

	println!("Seeding Roles....");
	let mut role_ids = Vec::new();
	for role in [RolesEnum::Admin, RolesEnum::Student, RolesEnum::Staf].iter() {
		let role_str = role.to_string();
		if let Some(existing) = roles::Entity::find()
			.filter(roles::Column::Name.eq(role_str.clone()))
			.one(db)
			.await?
		{
			println!("Role '{}' already exists. Skipping.", role_str);
			role_ids.push((role.clone(), existing.id));
			continue;
		}
		let role_id = Uuid::new_v4();
		let role_model = roles::ActiveModel {
			id: Set(role_id),
			name: Set(role_str.clone()),
			created_at: Set(Some(Utc::now())),
			updated_at: Set(Some(Utc::now())),
		};
		roles::Entity::insert(role_model).exec(db).await?;
		println!("Inserted role '{}'.", role_str);
		role_ids.push((role.clone(), role_id));
	}

	println!("Seeding Role Permissions (admin)....");
	let admin_role_id = role_ids
		.iter()
		.find(|(role, _)| *role == RolesEnum::Admin)
		.expect("Admin role should exist")
		.1;
	for (_, perm_id) in permission_ids.iter() {
		if let Some(_) = roles_permissions::Entity::find()
			.filter(roles_permissions::Column::RoleId.eq(admin_role_id))
			.filter(roles_permissions::Column::PermissionId.eq(*perm_id))
			.one(db)
			.await?
		{
			println!(
                "Role-Permission mapping for admin and permission {} already exists. Skipping.",
                perm_id
            );
			continue;
		}
		let role_permission = roles_permissions::ActiveModel {
			id: Set(Uuid::new_v4()),
			permission_id: Set(*perm_id),
			role_id: Set(admin_role_id),
		};
		roles_permissions::Entity::insert(role_permission)
			.exec(db)
			.await?;
		println!(
			"Inserted Role-Permission mapping for admin and permission {}.",
			perm_id
		);
	}

	println!("Seeding Session Tests....");
	let session_name = "Mid Term Exams Session";
	let session = if let Some(existing) = test_sessions::Entity::find()
		.filter(test_sessions::Column::SessionName.eq(session_name))
		.one(db)
		.await?
	{
		println!("Session '{}' already exists. Skipping.", session_name);
		existing
	} else {
		let session_id = Uuid::new_v4();
		let new_session = test_sessions::ActiveModel {
			id: Set(session_id),
			session_name: Set(session_name.to_owned()),
			category: Set("General".to_owned()),
			student_type: Set(Some("tni".to_string())),
			description: Set("Lorem ipsum".to_string()),
			is_active: Set(false),
			created_at: Set(Some(Utc::now())),
			updated_at: Set(Some(Utc::now())),
		};
		test_sessions::Entity::insert(new_session).exec(db).await?;
		println!("Inserted session '{}'.", session_name);
		test_sessions::Entity::find_by_id(session_id)
			.one(db)
			.await?
			.expect("Session should be inserted")
	};

	println!("Seeding Tests with Questions and Options....");
	for test_number in 1..=2 {
		let test_name = format!("Test {}", test_number);
		let test_entity = if let Some(existing) = tests::Entity::find()
			.filter(tests::Column::TestName.eq(&test_name))
			.one(db)
			.await?
		{
			println!("Test '{}' already exists. Skipping.", test_name);
			existing
		} else {
			let test_id = Uuid::new_v4();

			let new_test = tests::ActiveModel {
				id: Set(test_id),
				test_name: Set(test_name.clone()),
				created_at: Set(Some(Utc::now())),
				updated_at: Set(Some(Utc::now())),
				// If your model has other fields, set them accordingly.
				..Default::default()
			};
			tests::Entity::insert(new_test).exec(db).await?;
			println!("Inserted test '{}'.", test_name);
			tests::Entity::find_by_id(test_id)
				.one(db)
				.await?
				.expect("Test should be inserted")
		};

		println!(
			"Linking test '{}' with session '{}'",
			test_name, session.session_name
		);
		if let Some(_) = sessions_has_tests::Entity::find()
			.filter(sessions_has_tests::Column::SessionId.eq(session.id))
			.filter(sessions_has_tests::Column::TestId.eq(test_entity.id))
			.one(db)
			.await?
		{
			println!(
				"Link between session '{}' and test '{}' already exists. Skipping.",
				session.session_name, test_name
			);
		} else {
			let link = sessions_has_tests::ActiveModel {
				id: Set(Uuid::new_v4()),
				session_id: Set(session.id),
				test_id: Set(test_entity.id),
				start_date: Set(Some(Utc::now())),
				end_date: Set(None),
				weight: Set(Some("3".to_string())),
				multiplier: Set(Some("1.5".to_string())),
			};
			sessions_has_tests::Entity::insert(link).exec(db).await?;
			println!(
				"Created link between session '{}' and test '{}'.",
				session.session_name, test_name
			);
		}

		for question_number in 1..=20 {
			let question_text =
				format!("{} - Question {}", test_name, question_number);
			let question_entity = if let Some(existing) = questions::Entity::find()
				.filter(questions::Column::Question.eq(&question_text))
				.filter(questions::Column::TestId.eq(test_entity.id))
				.one(db)
				.await?
			{
				println!("Question '{}' already exists. Skipping.", question_text);
				existing
			} else {
				let question_id = Uuid::new_v4();
				let new_question = questions::ActiveModel {
					id: Set(question_id),
					test_id: Set(test_entity.id),
					question: Set(question_text.clone()),
					discussion: Set("Auto-generated question".to_owned()),
					image_url: Set(Some("https://example.com/image.jpg".to_owned())),
				};
				questions::Entity::insert(new_question).exec(db).await?;
				println!("Inserted question '{}'.", question_text);
				questions::Entity::find_by_id(question_id)
					.one(db)
					.await?
					.expect("Question should be inserted")
			};

			for option_number in 1..=4 {
				let option_text = format!("Option {}", option_number);
				if let Some(_) = options::Entity::find()
					.filter(options::Column::QuestionId.eq(question_entity.id))
					.filter(options::Column::Label.eq(&option_text))
					.one(db)
					.await?
				{
					println!(
						"Option '{}' for question '{}' already exists. Skipping.",
						option_text, question_text
					);
					continue;
				}
				let option = options::ActiveModel {
					id: Set(Uuid::new_v4()),
					question_id: Set(question_entity.id),
					label: Set(option_text.clone()),
					is_correct: Set(option_number == 1),
					image_url: Set(Some("https://example.com/image.jpg".to_owned())),
				};
				options::Entity::insert(option).exec(db).await?;
				println!(
					"Inserted option '{}' for question '{}'.",
					option_text, question_text
				);
			}
		}
	}

	println!("Seeding Admin User....");
	let admin_email = "admin@example.com";
	if let Some(_) = users::Entity::find()
		.filter(users::Column::Email.eq(admin_email))
		.one(db)
		.await?
	{
		println!(
			"Admin user with email '{}' already exists. Skipping.",
			admin_email
		);
	} else {
		let user_id = Uuid::new_v4();
		let hashed_password = hash_password("password123");
		let user = users::ActiveModel {
			id: Set(user_id),
			role_id: Set(admin_role_id),
			fullname: Set("John Doe".to_owned()),
			email: Set(admin_email.to_owned()),
			email_verified: Set(Some(Utc::now())),
			referral_code: Set(Some("REF123".to_owned())),
			referred_by: Set(Some("REF999".to_owned())),
			phone_number: Set("1234567890".to_owned()),
			password: Set(hashed_password.unwrap()),
			avatar: Set(None),
			birth_date: Set(Some(Utc::now())),
			gender: Set(Some("Male".to_owned())),
			religion: Set(Some("None".to_owned())),
			identity_number: Set(Some("ID123456".to_owned())),
			student_type: Set("Full-time".to_owned()),
			is_active: Set(true),
			is_deleted: Set(false),
			is_profile_completed: Set(true),
			created_at: Set(Some(Utc::now())),
			updated_at: Set(Some(Utc::now())),
		};
		users::Entity::insert(user).exec(db).await?;
		println!("Inserted admin user with email '{}'.", admin_email);
	}

	println!("Seeding 100 random users....");
	let mut rng = rand::thread_rng();
	for i in 1..=100 {
		let email = format!("user{}@example.com", i);
		if let Some(_) = users::Entity::find()
			.filter(users::Column::Email.eq(email.clone()))
			.one(db)
			.await?
		{
			println!("User with email '{}' already exists. Skipping.", email);
			continue;
		}
		let (_, random_role_id) = role_ids.choose(&mut rng).unwrap();
		let user_id = Uuid::new_v4();
		let hashed_password = hash_password("password123");
		let user = users::ActiveModel {
			id: Set(user_id),
			role_id: Set(*random_role_id),
			fullname: Set(format!("User {}", i)),
			email: Set(email.clone()),
			email_verified: Set(Some(Utc::now())),
			referral_code: Set(Some(format!("REF{}", i))),
			referred_by: Set(Some("REF999".to_owned())),
			phone_number: Set(format!("123456789{}", i)),
			password: Set(hashed_password.unwrap()),
			avatar: Set(None),
			birth_date: Set(Some(Utc::now())),
			gender: Set(Some("Other".to_owned())),
			religion: Set(Some("None".to_owned())),
			identity_number: Set(Some(format!("ID{:06}", i))),
			student_type: Set("Full-time".to_owned()),
			is_active: Set(true),
			is_deleted: Set(false),
			is_profile_completed: Set(true),
			created_at: Set(Some(Utc::now())),
			updated_at: Set(Some(Utc::now())),
		};
		users::Entity::insert(user).exec(db).await?;
		println!("Inserted random user with email '{}'.", email);
	}

	Ok(())
}
