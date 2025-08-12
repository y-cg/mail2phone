use crate::config::PushoverConfig;
use anyhow::Result;
use pushover_rs::{send_pushover_request, MessageBuilder};

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
    async fn send_notification<T: Notification>(&self, item: &T) -> Result<()> {
        let msg = MessageBuilder::new(&self.user_key, &self.api_token, item.message())
            .set_title(item.title())
            .build();

        let resp = send_pushover_request(msg)
            .await
            .map_err(|e| anyhow::anyhow!("Pushover request failed: {}", e))?;

        if let Some(error) = resp.errors {
            return Err(anyhow::anyhow!(
                "Pushover API returned errors: {}",
                error.join(", ")
            ));
        }

        Ok(())
    }
}
