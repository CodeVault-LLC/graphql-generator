use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub schema: String,
    pub output: String,
    pub plugin: String,
}

pub static CONFIG: Lazy<Config> =
    Lazy::new(|| Config::load_from_file("graphql-gen.json").expect("Failed to load configuration"));

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&file_content)?;
        Ok(config)
    }
}
