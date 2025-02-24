pub mod v1;

use crate::Config;

use axum::{
	http::{header, HeaderValue, Method},
	middleware::from_fn,
	response::Redirect,
	routing::get,
	Router,
};

use tower_http::cors::CorsLayer;
use utoipa_swagger_ui::SwaggerUi;

pub async fn root_routes() -> Router {
	let config = Config::new();

	let cors_origins = match config.rust_env.as_str() {
		"development" => vec!["http://localhost:5173"],
		"production" => {
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
		.nest("/sessions", v1::sessions::sessions_router())
		.nest("/tests", v1::tests::tests_router())
		.nest("/permissions", v1::permissions::permissions_router())
		.nest("/storage", v1::storage::storage_router().await)
		.layer(from_fn(v1::auth::auth_middleware::authorization_middleware));

	let v1_routes = Router::new()
		.merge(v1_public_routes)
		.merge(v1_protected_routes);

	let v2_routes = Router::new().route("/", get(|| async { "Coming Soon" }));

	Router::new()
		.route("/", get(Redirect::to("/docs")))
		.nest("/v1", v1_routes)
		.nest("/v2", v2_routes)
		.merge(SwaggerUi::new("/docs").url("/openapi.json", v1::docs::docs_router()))
		.layer(cors_middleware)
}
