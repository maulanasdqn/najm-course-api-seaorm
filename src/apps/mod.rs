mod auth;
mod permissions;
mod roles;
mod users;

use crate::utils::meta::{TMetaRequest, TMetaResponse};
use auth::{auth_middleware::authorization_middleware, auth_router};
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
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;


pub async fn routes() -> Router {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            auth::auth_controller::post_login,
            auth::auth_controller::post_register,
            auth::auth_controller::post_forgot,
            auth::auth_controller::post_verify_email,
            auth::auth_controller::post_send_otp,
            auth::auth_controller::post_new_password,
            users::users_controller::get_users,
            users::users_controller::get_detail_user,
            users::users_controller::post_create_user,
            users::users_controller::put_update_user,
            users::users_controller::delete_user,
            roles::roles_controller::get_roles,
            roles::roles_controller::get_detail_role,
            roles::roles_controller::post_create_role,
            roles::roles_controller::put_update_role,
            roles::roles_controller::delete_role,
            permissions::permissions_controller::get_permissions,
            permissions::permissions_controller::get_detail_permission,
            permissions::permissions_controller::post_create_permission,
            permissions::permissions_controller::put_update_permission,
            permissions::permissions_controller::delete_permission
        ),
        components(
            schemas(
                TMetaResponse,
                TMetaRequest,
                auth::auth_dto::AuthLoginDto,
                auth::auth_dto::AuthRegisterDto,
                auth::auth_dto::AuthResponse,
                auth::auth_dto::AuthTokenDto,
                auth::auth_dto::AuthDataDto,
                auth::auth_dto::AuthForgotDto,
                auth::auth_dto::AuthVerifyEmailDto,
                auth::auth_dto::AuthRequestNewPasswordDto,
                auth::auth_dto::MessageResponse,
                users::users_dto::UsersCreateRequestDto,
                users::users_dto::UsersUpdateRequestDto,
                users::users_dto::UsersListResponseDto,
                users::users_dto::UsersDetailResponseDto,
                users::users_dto::UsersItemDto,
                roles::roles_dto::RolesItemDto,
                roles::roles_dto::RolesRequestDto,
                roles::roles_dto::RolesListResponseDto,
                roles::roles_dto::RolesDetailResponseDto,
                permissions::permissions_dto::PermissionsItemDto,
                permissions::permissions_dto::PermissionsRequestDto,
                permissions::permissions_dto::PermissionsListResponseDto,
                permissions::permissions_dto::PermissionsDetailResponseDto
                
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
        _ => vec!["https://najmcourse.com", "https://cat.najmcourse.com"],
    };

    let allowed_origins: Vec<HeaderValue> = cors_origins
        .into_iter()
        .filter_map(|origin| origin.parse::<HeaderValue>().ok())
        .collect();

    let cors_middleware = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .route("/", get(|| async { Redirect::temporary("/api/docs") }))
        .nest(
            "/api",
            Router::new()
                .nest("", protected_routes().await)
                .nest("/auth", auth_router()),
        )
        .merge(SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/api/rapidoc"))
        .layer(cors_middleware)
}

async fn protected_routes() -> Router {
    Router::new()
        .nest("/users", users::users_router())
        .nest("/roles", roles::roles_router())
        .nest("/permissions", permissions::permissions_router())
        .layer(from_fn(authorization_middleware))
}
