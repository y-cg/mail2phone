mod pushover;

pub use pushover::Pushover;

use anyhow::Result;

/// Trait for sending notifications.
pub trait Notifier {
    /// Send a notification with a title and message.
    async fn send_notification<T: Notification>(&self, item: &T) -> Result<()>;
}

pub trait Notification {
    fn title(&self) -> &str;
    fn message(&self) -> &str;
}
