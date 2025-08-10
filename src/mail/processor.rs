use crate::notification::Notification;
use anyhow::Result;
use imap::types::Fetch;

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
        let title = fetch
            .envelope()
            .and_then(|env| env.subject)
            // convert &[u8] to String
            .map(|s| String::from_utf8_lossy(s).to_string())
            .unwrap_or_else(|| "No Subject".to_string());

        let body = fetch
            .body()
            .map(|s| String::from_utf8_lossy(s).to_string())
            .unwrap_or_else(|| "No Body".to_string());

        let summary = MailSummary {
            title,
            message: body,
        };

        Ok(summary)
    }
}
