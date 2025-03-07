use crate::Config;
use axum::{serve, Router};
use std::{future::Future, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub async fn axum_init<F, Fut>(router_fn: F)
where
	F: Fn() -> Fut,
	Fut: Future<Output = Router>,
{
	let config = Config::new();
	let port: u16 = config.port.parse().expect("Invalid port number");
	let addr = SocketAddr::from(([0, 0, 0, 0], port));
	let listener = TcpListener::bind(&addr).await.unwrap();
	println!("Listening on http://{}", addr);

	// Apply the TraceLayer middleware to log HTTP requests and responses
	let router = router_fn().await.layer(TraceLayer::new_for_http());

	match serve(listener, router).await {
		Ok(_) => println!("Server stopped gracefully."),
		Err(err) => println!("Server encountered an error: {}", err),
	}
}
