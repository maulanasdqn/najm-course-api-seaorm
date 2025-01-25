use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

#[derive(Clone)]
pub struct OtpManager {
    otps: Arc<Mutex<HashMap<String, (String, SystemTime)>>>,
    ttl: Duration,
}

impl OtpManager {
    pub fn new(ttl_secs: u64) -> Self {
        OtpManager {
            otps: Arc::new(Mutex::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    pub fn generate_otp(&self, identifier: &str) -> String {
        let otp: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        let mut otps = self.otps.lock().unwrap();
        otps.insert(identifier.to_string(), (otp.clone(), SystemTime::now()));

        otp
    }

    pub fn validate_otp(&self, identifier: &str, otp: &str) -> bool {
        let mut otps = self.otps.lock().unwrap();

        if let Some((stored_otp, timestamp)) = otps.get(identifier) {
            if stored_otp == otp && timestamp.elapsed().unwrap_or_default() <= self.ttl {
                otps.remove(identifier);
                return true;
            }
        }

        false
    }
}
