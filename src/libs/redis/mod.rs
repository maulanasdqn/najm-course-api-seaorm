use crate::Config;

pub fn connect_redis() -> redis::Connection {
	let config = Config::new();
	let redis_host_name = config.redis_hostname;
	let redis_conn_url = format!("redis://{}", redis_host_name);

	redis::Client::open(redis_conn_url)
		.expect("Invalid connection URL")
		.get_connection()
		.expect("Failed to connect to Redis")
}
