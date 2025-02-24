use najm_course_api::{axum_init, routes};

#[tokio::main]
async fn main() {
	axum_init(routes).await;
}
