pub mod apps;
pub mod libs;
pub mod utils;

pub use apps::v1::*;
pub use libs::database::*;
pub use libs::email::*;
pub use libs::otp::OtpManager;
pub use libs::redis::connect_redis;
pub use utils::dto::*;
pub use utils::error::AppError;
pub use utils::jwt::*;
pub use utils::password::*;
pub use utils::response::*;

pub fn get_version() -> Result<String, Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

    let manifest_path = std::path::Path::new(&manifest_dir).join("Cargo.toml");

    let manifest = std::fs::read_to_string(&manifest_path)?;

    let toml: toml::Value = toml::from_str(&manifest)?;

    if let Some(version) = toml
        .get("package")
        .and_then(|pkg| pkg.get("version"))
        .and_then(|v| v.as_str())
    {
        Ok(version.to_string())
    } else {
        Err("Failed to find the package version in Cargo.toml".into())
    }
}
