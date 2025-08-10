use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct ImapConfig {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PushoverConfig {
    pub user_key: String,
    pub api_token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub poll_interval_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub imap: ImapConfig,
    pub pushover: PushoverConfig,
    pub app: AppConfig,
}

impl Config {
    /// Loads the configuration from a TOML file at the given path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;
        let config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.as_ref().display()))?;
        Ok(config)
    }
}
