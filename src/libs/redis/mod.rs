use std::env;

pub fn connect_redis() -> redis::Connection {
    let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");

    let uri_scheme = if env::var("IS_TLS").is_ok() {
        "rediss"
    } else {
        "redis"
    };
    let redis_conn_url = format!("{}://{}", uri_scheme, redis_host_name);

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("Failed to connect to Redis")
}
