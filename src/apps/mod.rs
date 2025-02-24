use crate::Config;
use axum::{
	http::{header, HeaderValue, Method},
	response::Redirect,
	routing::get,
	Router,
};
use tower_http::cors::CorsLayer;
use utoipa_swagger_ui::SwaggerUi;

pub mod v1;
pub mod v2;
pub use v1::*;

pub async fn routes() -> Router {
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

	Router::new()
		.route("/", get(Redirect::to("/docs")))
		.nest("/v1", v1::routes().await)
		.nest("/v2", v2::routes().await)
		.merge(SwaggerUi::new("/docs").url("/openapi.json", v1::docs_router()))
		.layer(cors_middleware)
}
