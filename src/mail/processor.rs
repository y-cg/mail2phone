use crate::notification::Notification;
use anyhow::Result;
use mail_parser::Message;

pub trait Preprocessor {
    type Output: Notification;
    /// Process the email content before sending.
    fn preprocess(&self, msg: &Message) -> Result<Self::Output>;
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

    fn preprocess(&self, msg: &Message) -> Result<Self::Output> {
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
