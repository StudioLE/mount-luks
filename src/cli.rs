use crate::prelude::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mount_luks")]
#[command(about = "A CLI tool to unlock and mount LUKS encrypted disks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Default, Subcommand)]
enum Command {
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

pub fn cli() {
    let cli = Cli::parse();
    init_elapsed_logger();
    let command = cli.command.unwrap_or_default();
    let result = match command {
        Command::Mount => mount_command(),
        Command::Validate => validate_command(),
        Command::SetTpm => set_tpm_command(),
        Command::SetLuks => set_luks_command(),
    };
    if let Err(e) = result {
        print_error("Unable to continue");
        eprintln!("\n{e}");
    }
}
