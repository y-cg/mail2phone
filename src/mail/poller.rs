use std::net::TcpStream;

use super::processor::Preprocessor;
use crate::config::Config;
use crate::notification::Notifier;
use anyhow::{Context, Result};
use native_tls::{TlsConnector, TlsStream};

type Session = imap::Session<TlsStream<TcpStream>>;

/// Responsible for polling the IMAP server and processing new emails.
pub struct MailPoller<'a, N: Notifier, P: Preprocessor> {
    config: &'a Config,
    notifier: &'a N,
    processor: &'a P,
}

impl<'a, N: Notifier, P: Preprocessor> MailPoller<'a, N, P> {
    pub fn new(config: &'a Config, notifier: &'a N, processor: &'a P) -> Self {
        Self {
            config,
            notifier,
            processor,
        }
    }

    fn session(&self) -> Result<Session> {
        let tls = TlsConnector::builder()
            .build()
            .context("Failed to build TLS connector")?;

        let client = imap::connect(
            (self.config.imap.server.as_str(), self.config.imap.port),
            &self.config.imap.server,
            &tls,
        )
        .context("Failed to connect to IMAP server")?;

        client
            .login(&self.config.imap.username, &self.config.imap.password)
            .map_err(|(e, _)| anyhow::anyhow!("IMAP login failed: {}", e))
    }

    /// Polls the IMAP server and processes new unseen emails.
    pub fn poll(&self) -> Result<()> {
        let mut session = self.session()?;

        let messages = self
            .unseen_emails(&mut session)
            .context("Failed to fetch unseen emails")?;

        messages.iter().try_for_each(|item| {
            self.notifier
                .send_notification(item)
                .context("Failed to send notification")
        })?;

        // Ignoring logout errors is fine
        session.logout().ok();
        Ok(())
    }

    fn unseen_emails(&self, session: &mut Session) -> Result<Vec<P::Output>> {
        // Select the inbox
        session.select("INBOX").context("Failed to select INBOX")?;

        // Find all unseen emails and mark them as seen by using the SEARCH command
        // with the UNSEEN criteria, then fetching with the RFC822 format
        let unseen_sequence = session
            .search("UNSEEN")
            .context("Failed to search for unseen emails")?;

        // Convert sequence set to string format for fetch command
        let sequence_set = unseen_sequence
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        // Fetch messages and mark them as seen
        // (removing the \Seen flag happens automatically when fetching with RFC822)
        let messages = session
            .fetch(&sequence_set, "RFC822")
            .context("Failed to fetch unseen emails")?;

        let notifications = messages
            .iter()
            .map(|fetch| {
                self.processor
                    .preprocess(fetch)
                    .context("Failed to process email")
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(notifications)
    }
}
