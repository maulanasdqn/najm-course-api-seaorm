pub mod apps;
pub mod config;
pub mod libs;
pub mod utils;

pub use apps::*;
pub use config::*;
pub use libs::*;
pub use utils::*;

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
