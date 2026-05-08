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

/// Available CLI subcommands.
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
        eprintln!("\n{}", e.render());
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn cli_internal() -> Result<(), StructuredError> {
    let cli = Cli::parse();
    init_elapsed_logger(cli.log_level);
    let options = Options::read_options(cli.config).map_err(StructuredError::from)?;
    let command = cli.command.unwrap_or_default();
    if options.no_header != Some(true) {
        print_header(&options, command);
    }
    let services = services(options);
    match command {
        SubCommand::Mount => services
            .get::<MountHandler>()
            .map_err(StructuredError::from)?
            .execute(),
        SubCommand::Validate => services
            .get::<ValidateHandler>()
            .map_err(StructuredError::from)?
            .execute(),
        SubCommand::SetTpm => services
            .get::<SetTpmHandler>()
            .map_err(StructuredError::from)?
            .execute(),
        SubCommand::SetLuks => services
            .get::<SetLuksHandler>()
            .map_err(StructuredError::from)?
            .execute(),
    }
}

/// Build the dependency injection container.
fn services(options: Options) -> ServiceProvider {
    ServiceBuilder::new()
        .with_instance(options)
        .with_trait::<dyn IsRoot, IsRootAdapter>()
        .with_trait::<dyn IsLuks, CryptsetupAdapter>()
        .with_trait::<dyn Unlock, CryptsetupAdapter>()
        .with_trait::<dyn CheckKey, CryptsetupAdapter>()
        .with_trait::<dyn AddKey, CryptsetupAdapter>()
        .with_trait::<dyn Unseal, Tpm2Adapter>()
        .with_trait::<dyn GetHandles, Tpm2Adapter>()
        .with_trait::<dyn CreatePolicy, Tpm2Adapter>()
        .with_trait::<dyn CreatePrimary, Tpm2Adapter>()
        .with_trait::<dyn CreateObject, Tpm2Adapter>()
        .with_trait::<dyn LoadObject, Tpm2Adapter>()
        .with_trait::<dyn Persist, Tpm2Adapter>()
        .with_trait::<dyn FindMount, FindmntAdapter>()
        .with_trait::<dyn MountPartition, MountAdapter>()
        .with_trait::<dyn PromptPassword, RpasswordAdapter>()
        .with_trait::<dyn GetKey, GetKeyAdapter>()
        .with_trait::<dyn ResolvePartition, ResolvePartitionAdapter>()
        .with_trait::<dyn CheckMountExists, MountExistsAdapter>()
        .with_trait::<dyn IsPartitionLocked, PartitionLockedAdapter>()
        .with_type::<MountHandler>()
        .with_type::<ValidateHandler>()
        .with_type::<SetTpmHandler>()
        .with_type::<SetLuksHandler>()
        .build()
}
