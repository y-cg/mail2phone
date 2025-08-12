mod config;
mod mail;
mod notification;

use anyhow::Context;
use clap::Parser;
use mail::processor::MailProcessor;

use crate::config::Config;
use crate::mail::poller::MailPoller;
use crate::notification::Pushover;
use anyhow::Result;
use std::time::Duration;

/// CLI arguments for mail2phone
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the configuration file
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Load configuration
    let config = Config::from_file(&cli.config).context("Failed to load configuration")?;

    // Initialize notifier (Pushover)
    let notifier = Pushover::from_config(&config.pushover);

    // Create EnvelopeProcessor
    let processor = MailProcessor::new();

    // Create MailChecker (generic over Notifier and EnvelopeProcessor)
    let poller = MailPoller::new(&config, &notifier, &processor);

    // Start polling loop
    loop {
        if let Err(e) = poller.poll().await {
            eprintln!("Error: {:#}", e);
        }
        tokio::time::sleep(Duration::from_secs(config.app.poll_interval_seconds)).await;
    }
}
