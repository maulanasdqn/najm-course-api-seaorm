use najm_course_api::{apps::routes, libs::axum::run};

#[tokio::main]
async fn main() {
    run(routes).await;
}
