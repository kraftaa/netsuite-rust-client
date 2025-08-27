use anyhow::Result;
use config::{Config as ConfigBuilder, Environment, File};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub netsuite: NetSuiteConfig,
}

#[derive(Debug, Deserialize)]
pub struct NetSuiteConfig {
    pub account_id: String,
    pub consumer_key: String,
    pub consumer_secret: String,
    pub token_id: String,
    pub token_secret: String,
    pub base_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // Load .env file if it exists
        dotenv::dotenv().ok();
        
        let config = ConfigBuilder::builder()
            // Start with default config
            .add_source(File::from(Path::new("config/default.toml")).required(false))
            // Add environment-specific config
            .add_source(File::from(Path::new("config/local.toml")).required(false))
            // Add environment variables
            .add_source(Environment::with_prefix("NETSUITE").separator("_"))
            .build()?;
        
        let config: AppConfig = config.try_deserialize()?;
        Ok(config)
    }
}

impl Default for NetSuiteConfig {
    fn default() -> Self {
        Self {
            account_id: "".to_string(),
            consumer_key: "".to_string(),
            consumer_secret: "".to_string(),
            token_id: "".to_string(),
            token_secret: "".to_string(),
            base_url: "https://rest.na1.netsuite.com".to_string(),
        }
    }
}
