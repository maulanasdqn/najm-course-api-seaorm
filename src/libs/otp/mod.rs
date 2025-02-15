use rand::{thread_rng, Rng};
use redis::Commands;
use std::time::Duration;

pub struct OtpManager {
	ttl: Duration,
}

impl OtpManager {
	pub fn new(ttl_secs: u64) -> Self {
		OtpManager {
			ttl: Duration::from_secs(ttl_secs),
		}
	}

	pub fn generate_otp(
		&self,
		mut redis_conn: redis::Connection,
		identifier: &str,
	) -> u32 {
		let otp: u32 = thread_rng().gen_range(100_000..1_000_000);

		let key = format!("otp:{}", identifier);
		let _: () = redis_conn
			.set_ex(
				key.clone(),
				otp.to_string(),
				(self.ttl.as_secs() as usize).try_into().unwrap(),
			)
			.expect("Failed to store OTP in Redis");

		otp
	}

	pub fn validate_otp(
		&self,
		mut redis_conn: redis::Connection,
		identifier: &str,
		otp: u32,
	) -> bool {
		let key = format!("otp:{}", identifier);
		if let Ok(stored_otp) = redis_conn.get::<_, String>(&key) {
			if stored_otp == otp.to_string() {
				let _: () = redis_conn
					.del(&key)
					.expect("Failed to delete OTP from Redis");
				return true;
			}
		}
		false
	}
}
