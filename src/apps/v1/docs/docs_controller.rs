use crate::apps::v1;

use crate::apps::v1::{
	AuthChangePasswordRequestDto, AuthDataDto, AuthForgotRequestDto,
	AuthLoginRequestDto, AuthNewPasswordRequestDto, AuthRefreshTokenRequestDto,
	AuthRegisterRequestDto, AuthTokenItemDto, AuthVerifyEmailRequestDto,
	OptionsItemDto, OptionsRequestCreateDto, PermissionsItemDto,
	PermissionsRequestDto, QuestionsItemDto, RolesItemDto, RolesItemListDto,
	RolesRequestCreateDto, RolesRequestUpdateDto, SessionsItemDto,
	SessionsItemListDto, SessionsRequestCreateDto, SessionsRequestUpdateDto,
	StorageRequestDto, StorageResponseDto, TestsItemDto, TestsItemListDto,
	TestsRequestCreateDto, TestsRequestUpdateDto, UsersActiveInactiveRequestDto,
	UsersCreateRequestDto, UsersItemDto, UsersItemListDto, UsersUpdateRequestDto,
};

use crate::{
	MessageResponseDto, MetaRequestDto, MetaResponseDto, QuestionsRequestCreateDto,
	ResponseSuccessDto, ResponseSuccessListDto, TestAnswersItemDto,
	TestAnswersRequestCreateDto,
};

use utoipa::{
	openapi::security::{Http, HttpAuthScheme, SecurityScheme},
	Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        v1::auth::auth_controller::post_login,
        v1::auth::auth_controller::post_register,
        v1::auth::auth_controller::post_forgot,
        v1::auth::auth_controller::post_verify_email,
        v1::auth::auth_controller::post_send_otp,
        v1::auth::auth_controller::post_new_password,
        v1::auth::auth_controller::post_change_password,
        v1::auth::auth_controller::post_refresh,

        v1::users::users_controller::get_users,
        v1::users::users_controller::get_detail_user,
        v1::users::users_controller::get_user_me,
        v1::users::users_controller::put_update_user_me,
        v1::users::users_controller::post_create_user,
        v1::users::users_controller::put_update_user,
        v1::users::users_controller::put_activate_user,
        v1::users::users_controller::delete_user,

        v1::roles::roles_controller::get_roles,
        v1::roles::roles_controller::get_detail_role,
        v1::roles::roles_controller::post_create_role,
        v1::roles::roles_controller::put_update_role,
        v1::roles::roles_controller::delete_role,

        v1::permissions::permissions_controller::get_permissions,
        v1::permissions::permissions_controller::get_detail_permission,
        v1::permissions::permissions_controller::post_create_permission,
        v1::permissions::permissions_controller::put_update_permission,
        v1::permissions::permissions_controller::delete_permission,

        v1::sessions::sessions_controller::get_sessions,
        v1::sessions::sessions_controller::get_detail_session,
        v1::sessions::sessions_controller::post_create_session,
        v1::sessions::sessions_controller::put_update_session,
        v1::sessions::sessions_controller::delete_session,

        v1::tests::tests_controller::get_tests,
        v1::tests::tests_controller::get_detail_test,
        v1::tests::tests_controller::post_create_test,
        v1::tests::tests_controller::put_update_test,
        v1::tests::tests_controller::delete_test,
        v1::tests::tests_controller::get_test_answer,
        v1::tests::tests_controller::post_create_test_answer,
        v1::tests::tests_controller::delete_test_answer,

        v1::storage::storage_controller::post_upload,
    ),
    components(
        schemas(
           MetaRequestDto,
           MetaResponseDto,
           MessageResponseDto,

           ResponseSuccessDto<AuthTokenItemDto>,
           ResponseSuccessDto<AuthDataDto>,

           ResponseSuccessDto<UsersItemDto>,
           ResponseSuccessListDto<UsersItemListDto>,

           ResponseSuccessListDto<RolesItemListDto>,
           ResponseSuccessDto<RolesItemDto>,

           ResponseSuccessListDto<PermissionsItemDto>,
           ResponseSuccessDto<PermissionsItemDto>,

           ResponseSuccessListDto<SessionsItemListDto>,
           ResponseSuccessDto<SessionsItemDto>,

           ResponseSuccessListDto<TestsItemListDto>,
           ResponseSuccessDto<TestsItemDto>,

           ResponseSuccessDto<TestAnswersItemDto>,

           ResponseSuccessDto<StorageResponseDto>,

           StorageRequestDto,
           StorageResponseDto,

           AuthLoginRequestDto,
           AuthRegisterRequestDto,
           AuthTokenItemDto,
           AuthDataDto,
           AuthForgotRequestDto,
           AuthVerifyEmailRequestDto,
           AuthNewPasswordRequestDto,
           AuthChangePasswordRequestDto,
           AuthRefreshTokenRequestDto,

           UsersCreateRequestDto,
           UsersActiveInactiveRequestDto,
           UsersUpdateRequestDto,
           UsersItemDto,

           RolesItemDto,
           RolesItemListDto,
           RolesRequestCreateDto,
           RolesRequestUpdateDto,

           PermissionsItemDto,
           PermissionsRequestDto,

           SessionsItemDto,
           SessionsItemListDto,
           SessionsRequestCreateDto,
           SessionsRequestUpdateDto,

           TestsItemDto,
           TestsItemListDto,
           TestsRequestCreateDto,
           TestsRequestUpdateDto,
           TestAnswersItemDto,
           TestAnswersRequestCreateDto,
           QuestionsRequestCreateDto,
           OptionsRequestCreateDto,
           QuestionsItemDto,
           OptionsItemDto,
        )
    ),
    info(
        title = "Najm Course API",
        description = "Najm Course API Documentation",
        version = "0.1.0",
        contact(
            name = "Maulana Sodiqin",
            url = ""
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Authentication", description = "List of Authentication Endpoints"),
        (name = "Users", description = "List of Users Endpoints")
    )
)]

pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
	fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
		if let Some(components) = openapi.components.as_mut() {
			components.add_security_scheme(
				"Bearer",
				SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
			);
		}
	}
}
