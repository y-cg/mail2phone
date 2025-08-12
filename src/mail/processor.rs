use crate::notification::Notification;
use anyhow::{Context, Result};
use imap::types::Fetch;
use mail_parser::MessageParser;

pub trait Preprocessor {
    type Output: Notification;
    /// Process the email content before sending.
    fn preprocess(&self, fetch: &Fetch) -> Result<Self::Output>;
}

pub struct MailProcessor {
    // todo
}

pub struct MailSummary {
    pub title: String,
    pub message: String,
}

impl Notification for MailSummary {
    fn title(&self) -> &str {
        &self.title
    }

    fn message(&self) -> &str {
        &self.message
    }
}

impl MailProcessor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Preprocessor for MailProcessor {
    type Output = MailSummary;

    fn preprocess(&self, fetch: &Fetch) -> Result<Self::Output> {
        let body = fetch
            .body()
            .ok_or_else(|| anyhow::anyhow!("No body found in mail"))?;

        let msg = MessageParser::default()
            .parse(body)
            .context("Fail to parse mail")?;

        let summary = MailSummary {
            title: msg
                .subject()
                .map_or_else(|| "No Subject".to_string(), |s| s.to_string()),
            message: msg
                .body_text(0)
                .map_or_else(|| "No Body".to_string(), |s| s.to_string()),
        };

        Ok(summary)
    }
}
