pub mod v1;

use crate::{
    utils::dto::{MessageResponseDto, MetaRequestDto, MetaResponseDto},
    ResponseSuccessDto,
};
use axum::{
    http::{header, HeaderValue, Method},
    middleware::from_fn,
    response::Redirect,
    routing::get,
    Router,
};
use std::env;
use tower_http::cors::CorsLayer;
use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;
use v1::auth::{AuthDataDto, AuthTokenItemDto};

pub async fn root_routes() -> Router {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            v1::auth::auth_controller::post_login,
            v1::auth::auth_controller::post_register,
            v1::auth::auth_controller::post_forgot,
            v1::auth::auth_controller::post_verify_email,
            v1::auth::auth_controller::post_send_otp,
            v1::auth::auth_controller::post_new_password,
            v1::auth::auth_controller::post_refresh,
            v1::users::users_controller::get_users,
            v1::users::users_controller::get_detail_user,
            v1::users::users_controller::post_create_user,
            v1::users::users_controller::put_update_user,
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
            v1::permissions::permissions_controller::delete_permission
        ),
        components(
            schemas(
                MetaRequestDto,
                MetaResponseDto,
                MessageResponseDto,
                ResponseSuccessDto<AuthTokenItemDto>,
                ResponseSuccessDto<AuthDataDto>,
                v1::auth::auth_dto::AuthLoginRequestDto,
                v1::auth::auth_dto::AuthRegisterRequestDto,
                v1::auth::auth_dto::AuthTokenItemDto,
                v1::auth::auth_dto::AuthDataDto,
                v1::auth::auth_dto::AuthForgotRequestDto,
                v1::auth::auth_dto::AuthVerifyEmailRequestDto,
                v1::auth::auth_dto::AuthNewPasswordRequestDto,
                v1::auth::auth_dto::AuthRefreshTokenRequestDto,
                v1::users::users_dto::UsersCreateRequestDto,
                v1::users::users_dto::UsersUpdateRequestDto,
                v1::users::users_dto::UsersListResponseDto,
                v1::users::users_dto::UsersDetailResponseDto,
                v1::users::users_dto::UsersItemDto,
                v1::roles::roles_dto::RolesItemDto,
                v1::roles::roles_dto::RolesRequestDto,
                v1::roles::roles_dto::RolesListResponseDto,
                v1::roles::roles_dto::RolesDetailResponseDto,
                v1::permissions::permissions_dto::PermissionsItemDto,
                v1::permissions::permissions_dto::PermissionsRequestDto,
                v1::permissions::permissions_dto::PermissionsListResponseDto,
                v1::permissions::permissions_dto::PermissionsDetailResponseDto
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
                url = "https://opensource.org/licenses/MIT",
            )
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "Authentication", description = "Authentication Endpoint"),
        )
    )]
    struct ApiDoc;

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

    let cors_origins = match env::var("RUST_ENV").as_deref() {
        Ok("development") => vec!["http://localhost:5173"],
        Ok("production") => {
            vec!["https://najmcourse.com", "https://cat.najmcourse.com"]
        }
        _ => vec![
            "http://localhost:5173",
            "https://najmcourse.com",
            "https://cat.najmcourse.com",
        ],
    };

    let allowed_origins: Vec<HeaderValue> = cors_origins
        .into_iter()
        .filter_map(|origin| origin.parse::<HeaderValue>().ok())
        .collect();

    let cors_middleware = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true);

    let v1_public_routes = Router::new().nest("/auth", v1::auth::auth_router());

    let v1_protected_routes = Router::new()
        .nest("/users", v1::users::users_router())
        .nest("/roles", v1::roles::roles_router())
        .nest("/permissions", v1::permissions::permissions_router())
        .layer(from_fn(v1::auth::auth_middleware::authorization_middleware));

    let v1_routes = Router::new()
        .merge(v1_public_routes)
        .merge(v1_protected_routes);

    let v2_routes = Router::new().route("/", get(|| async { "Comming Soon" }));

    Router::new()
        .route("/", get(Redirect::to("/docs")))
        .nest("/v1", v1_routes)
        .nest("/v2", v2_routes)
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .layer(cors_middleware)
}
