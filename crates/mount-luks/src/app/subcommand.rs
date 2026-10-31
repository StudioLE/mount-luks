use crate::prelude::*;
use strum::Display;
use usage_rs::Subcommands as UsageSubcommands;

/// Available CLI subcommands.
#[derive(Copy, Clone, Debug, Default, Display, Eq, PartialEq, UsageSubcommands)]
pub enum Subcommand {
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

/// Dispatch the selected [`Subcommand`] to its handler.
#[derive(FromServices)]
pub struct SubcommandHandler {
    cli: Arc<CliArgs>,
    mount: Arc<MountHandler>,
    validate: Arc<ValidateHandler>,
    set_tpm: Arc<SetTpmHandler>,
    set_luks: Arc<SetLuksHandler>,
}

impl SubcommandHandler {
    /// Execute the selected subcommand.
    pub fn run(&self) -> Result<(), StructuredError> {
        let command = self.cli.command.unwrap_or_default();
        match command {
            Subcommand::Mount => self.mount.execute(),
            Subcommand::Validate => self.validate.execute(),
            Subcommand::SetTpm => self.set_tpm.execute(),
            Subcommand::SetLuks => self.set_luks.execute(),
        }
    }
}
