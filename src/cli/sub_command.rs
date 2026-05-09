use crate::prelude::*;
use clap::Subcommand;
use strum::Display;

/// Available CLI subcommands.
#[derive(Copy, Clone, Default, Display, Subcommand)]
pub enum SubCommand {
    /// Unlock and mount a LUKS encrypted partition
    #[default]
    Mount,
    /// Validate the LUKS passphrase without unlocking
    Validate,
    /// Seal the passphrase component in the TPM
    SetTpm,
    /// Add the passphrase to LUKS
    SetLuks,
}

/// Dispatch the selected [`SubCommand`] to its handler.
#[derive(FromServices)]
pub struct SubCommandHandler {
    cli: Arc<CliOptions>,
    mount: Arc<MountHandler>,
    validate: Arc<ValidateHandler>,
    set_tpm: Arc<SetTpmHandler>,
    set_luks: Arc<SetLuksHandler>,
}

impl SubCommandHandler {
    /// Execute the selected subcommand.
    pub fn run(&self) -> Result<(), StructuredError> {
        let command = self.cli.command.unwrap_or_default();
        match command {
            SubCommand::Mount => self.mount.execute(),
            SubCommand::Validate => self.validate.execute(),
            SubCommand::SetTpm => self.set_tpm.execute(),
            SubCommand::SetLuks => self.set_luks.execute(),
        }
    }
}
