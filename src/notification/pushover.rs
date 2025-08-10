use crate::config::PushoverConfig;
use anyhow::{Context, Result};

use super::{Notification, Notifier};

/// Pushover provider
pub struct Pushover {
    api_token: String,
    user_key: String,
}

impl Pushover {
    /// Create a new Pushover notifier from config.
    pub fn from_config(config: &PushoverConfig) -> Self {
        Self {
            api_token: config.api_token.clone(),
            user_key: config.user_key.clone(),
        }
    }
}

impl Notifier for Pushover {
    fn send_notification<T: Notification>(&self, item: &T) -> Result<()> {
        let client = reqwest::blocking::Client::new();
        let params = [
            ("token", self.api_token.as_str()),
            ("user", self.user_key.as_str()),
            ("title", item.title()),
            ("message", item.message()),
        ];

        let resp = client
            .post("https://api.pushover.net/1/messages.json")
            .form(&params)
            .send()
            .context("Failed to send pushover request")?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Pushover API error: {}", resp.status()))
        }
    }
}
