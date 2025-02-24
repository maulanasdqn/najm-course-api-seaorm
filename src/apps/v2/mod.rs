use axum::{routing::get, Router};

pub async fn routes() -> Router {
	Router::new().route("/", get(|| async { "Coming Soon" }))
}
