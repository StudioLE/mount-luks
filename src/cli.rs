use crate::prelude::*;
use clap::{Parser, Subcommand};
use std::process::ExitCode;
use strum::Display;

#[derive(Parser)]
#[command(name = APP_NAME)]
#[command(about = "A CLI tool to unlock and mount LUKS encrypted disks", long_about = None)]
struct Cli {
    /// Path to a specific config file
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Set the log level
    #[arg(long, value_enum)]
    pub log_level: Option<LogLevel>,

    #[command(subcommand)]
    pub command: Option<SubCommand>,
}

#[derive(Copy, Clone, Default, Display, Subcommand)]
pub enum SubCommand {
    /// Unlock and mount a LUKS encrypted partition
    #[default]
    Mount,
    /// Check the key
    Validate,
    /// Save the TPM component of the passphrase in TPM
    SetTpm,
    /// Add the passphrase to LUKS
    SetLuks,
}

#[must_use]
pub fn cli() -> ExitCode {
    if let Err(e) = cli_internal() {
        print_error("Unable to continue");
        eprintln!("\n{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn cli_internal() -> Result<(), AnyReport> {
    let cli = Cli::parse();
    init_elapsed_logger(cli.log_level);
    let options = Options::read_options(cli.config)?;
    let command = cli.command.unwrap_or_default();
    if options.no_header != Some(true) {
        print_header(&options, command);
    }
    match command {
        SubCommand::Mount => mount_command(options),
        SubCommand::Validate => validate_command(options),
        SubCommand::SetTpm => set_tpm_command(options),
        SubCommand::SetLuks => set_luks_command(options),
    }
}
