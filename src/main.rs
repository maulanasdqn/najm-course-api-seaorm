use najm_course_api::{apps::root_routes, libs::axum::run};

#[tokio::main]
async fn main() {
    run(root_routes).await;
}
