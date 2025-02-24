use axum::{middleware::from_fn, Router};

pub mod auth;
pub mod docs;
pub mod permissions;
pub mod roles;
pub mod sessions;
pub mod storage;
pub mod tests;
pub mod users;

pub use auth::*;
pub use docs::*;
pub use permissions::*;
pub use roles::*;
pub use sessions::*;
pub use storage::*;
pub use tests::*;
pub use users::*;

pub async fn routes() -> Router {
	let public_routes = Router::new().nest("/auth", auth::auth_router());

	let protected_routes = Router::new()
		.nest("/users", users::users_router())
		.nest("/roles", roles::roles_router())
		.nest("/sessions", sessions::sessions_router())
		.nest("/tests", tests::tests_router())
		.nest("/permissions", permissions::permissions_router())
		.nest("/storage", storage::storage_router().await)
		.layer(from_fn(auth::authorization_middleware));

	Router::new().merge(public_routes).merge(protected_routes)
}
