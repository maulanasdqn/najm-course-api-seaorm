use najm_course_api::{apps::root_routes, axum_init};

#[tokio::main]
async fn main() {
	axum_init(root_routes).await;
}
