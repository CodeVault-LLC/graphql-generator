use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use log::{debug, error};

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
        let path: &Path = path.as_ref();

        // Check if the file exists
        if !path.exists() {
            error!("Error: Configuration file '{}' not found.", path.display());
            std::process::exit(1);
        }

        // Attempt to read the file
        let file_content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                error!(
                    "Error: Failed to read configuration file '{}': {}",
                    path.display(),
                    e
                );
                std::process::exit(1);
            }
        };

        // Attempt to parse the JSON
        let config: Config = match serde_json::from_str(&file_content) {
            Ok(config) => config,
            Err(e) => {
                error!(
                    "Error: Failed to parse configuration file '{}': {}",
                    path.display(),
                    e
                );
                std::process::exit(1);
            }
        };

        // Validate required fields
        if config.schema.is_empty() || config.output.is_empty() || config.plugin.is_empty() {
            error!(
                "Error: Missing required fields in configuration file '{}'.",
                path.display()
            );
            std::process::exit(1);
        }

        debug!("Configuration loaded successfully: {:?}", config);
        Ok(config)
    }
}
