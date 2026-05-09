use crate::prelude::*;
use clap::Parser;
use std::convert::Infallible;

#[derive(Parser)]
#[command(name = APP_NAME)]
#[command(about = "A CLI tool to unlock and mount LUKS encrypted disks", long_about = None)]
/// Parsed CLI arguments.
pub struct CliOptions {
    /// Path to a specific config file
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Set the log level
    #[arg(long, value_enum)]
    pub log_level: Option<LogLevel>,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<SubCommand>,
}

impl FromServices for CliOptions {
    type Error = Infallible;

    fn from_services(_: &ServiceProvider) -> Result<Self, Report<Self::Error>>
    where
        Self: Sized,
    {
        Ok(CliOptions::parse())
    }
}
